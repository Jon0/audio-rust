use std::ptr;
use std::ffi::CStr;
use std::ffi::CString;
use std::cmp::min;
use libc::c_int;
use libc::c_uint;
use libc::c_void;

use device::alsa::*;


pub type SndSize = snd_pcm_uframes_t;


pub struct SndError {
    name: String,
    err: c_int,
}


impl SndError {
    pub fn new(n: &str, e: c_int) -> SndError {
        SndError { name: String::from(n), err: e }
    }


    pub fn as_string(&self) -> String {
        unsafe {
            return CStr::from_ptr(snd_strerror(self.err)).to_string_lossy().into_owned();
        }
    }
}


pub struct Format {
    channels: i32,
    rate: i32,
}


impl Format {
    pub fn new(ch: i32, r: i32) -> Format {
        Format { channels: ch, rate: r }
    }


    pub fn format_id(&self) -> snd_pcm_format_t {
        return SND_PCM_FORMAT_S16_LE;
    }
}


pub struct Params {
    hw_params: *mut snd_pcm_hw_params_t,
}


impl Params {
    pub fn new() -> Result<Params, SndError> {
        let mut param_ptr: *mut snd_pcm_hw_params_t = ptr::null_mut();
        let mut err = 0;
        unsafe {
            let err = snd_pcm_hw_params_malloc(&mut param_ptr);
        }
        if err < 0 {
            return Err(SndError::new("snd_pcm_hw_params_malloc", err));
        }
        else {
            return Ok(Params { hw_params: param_ptr });
        }
    }

    pub fn buffer_size(&self) -> Result<SndSize, SndError> {
        let mut size: snd_pcm_uframes_t = 0;
        let mut err = 0;
        unsafe {
            err = snd_pcm_hw_params_get_buffer_size(self.hw_params, &mut size);
        }
        if err < 0 {
            return Err(SndError::new("snd_pcm_hw_params_get_buffer_size", err));
        }
        else {
            println!("Buffer size: {}", size);
            return Ok(size);
        }
    }


    pub fn any(&self, pcm: *mut snd_pcm_t) -> Option<SndError> {
        let mut err = 0;
        unsafe {
            err = snd_pcm_hw_params_any(pcm, self.hw_params);
        }
        if err < 0 {
            return Some(SndError::new("snd_pcm_hw_params_any", err));
        }
        else {
            return None;
        }
    }


    pub fn format(&self, pcm: *mut snd_pcm_t) -> Option<SndError> {
        let mut err = 0;
        let mut rate: c_uint = 44100;
        unsafe {
            err = snd_pcm_hw_params_set_rate_resample(pcm, self.hw_params, 1);
            err = snd_pcm_hw_params_set_access(pcm, self.hw_params, SND_PCM_ACCESS_RW_INTERLEAVED);
            err = snd_pcm_hw_params_set_format(pcm, self.hw_params, SND_PCM_FORMAT_S16_LE);
            err = snd_pcm_hw_params_set_channels(pcm, self.hw_params, 1);
            err = snd_pcm_hw_params_set_rate_near(pcm, self.hw_params, &mut rate, ptr::null_mut());
        }
        if err < 0 {
            return Some(SndError::new("snd_pcm_hw_params_any", err));
        }
        else {
            return None;
        }
    }



    pub fn apply(&self, pcm: *mut snd_pcm_t) -> Option<SndError> {
        let mut err = 0;
        unsafe {
            err = snd_pcm_hw_params(pcm, self.hw_params);
        }
        if err < 0 {
            return Some(SndError::new("snd_pcm_hw_params", err));
        }
        else {
            return None;
        }
    }
}


pub struct Device {
    pcm: *mut snd_pcm_t,
}


impl Device {
    pub fn open(name: &str) -> Result<Device, SndError> {
        let mut pcm_ptr: *mut snd_pcm_t = ptr::null_mut();
        let devname = CString::new(name).unwrap();
        let mut err = 0;
        unsafe {
            err = snd_pcm_open(&mut pcm_ptr, devname.as_ptr(), SND_PCM_STREAM_PLAYBACK, 0);
        }
        if err < 0 {
            return Err(SndError::new("snd_pcm_open", err));
        }
        else {
            return Ok(Device { pcm: pcm_ptr });
        }
    }


    pub fn setup(&self, params: &mut Params) -> Option<SndError> {
        match params.any(self.pcm) {
            Some(e) => return Some(e),
            None => {},
        }
        match params.format(self.pcm) {
            Some(e) => return Some(e),
            None => {},
        }
        return params.apply(self.pcm);
    }


    pub fn prepare(&self) -> Option<SndError> {
        let mut err = 0;
        unsafe {
            err = snd_pcm_prepare(self.pcm);
        }
        if err < 0 {
            return Some(SndError::new("snd_pcm_prepare", err));
        }
        else {
            return None;
        }
    }


    pub fn blocking(&self, block: bool) -> Result<i32, SndError> {
        let mut result = 0;
        unsafe {
            if (block) {
                result = snd_pcm_nonblock(self.pcm, 0);
            }
            else {
                result = snd_pcm_nonblock(self.pcm, SND_PCM_NONBLOCK);
            }
        }
        if result < 0 {
            return Err(SndError::new("snd_pcm_nonblock", result));
        }
        else {
            return Ok(result);
        }
    }


    pub fn wait(&self) -> Result<i32, SndError> {
        let mut result = 0;
        unsafe {
            result = snd_pcm_wait(self.pcm, -1);
        }
        if result < 0 {
            return Err(SndError::new("snd_pcm_wait", result));
        }
        else {
            return Ok(result);
        }
    }


    pub fn write_some(&self, data: &[i16]) -> Result<usize, SndError> {
        let mut size = 0;
        unsafe {
            size = snd_pcm_writei(self.pcm, data.as_ptr() as *const c_void, data.len() as snd_pcm_uframes_t);
        }
        if size < 0 {
            return Err(SndError::new("snd_pcm_writei", size as i32));
        }
        else {
            return Ok(size as usize);
        }
    }


    pub fn play(&self, data: &[i16]) -> Result<SndSize, SndError> {
        let available = data.len();
        let mut written: usize = 0;
        while written < available {
            let subdata = &data[written..min(written + 21024, data.len())];
            self.wait();
            match self.write_some(subdata) {
                Ok(count) => written += count,
                Err(err) => return Err(err),
            }
        }
        return Ok(written as SndSize);
    }
}
