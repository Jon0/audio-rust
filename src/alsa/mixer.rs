use std::ptr;
use std::ffi::CStr;
use std::ffi::CString;
use std::cmp::min;
use libc::c_int;
use libc::c_uint;
use libc::c_void;


use alsa::ffi::*;
use player::player::*;



fn create_error_string(error_code: c_int) -> String {
    unsafe {
        return CStr::from_ptr(snd_strerror(error_code)).to_string_lossy().into_owned();
    }
}


fn create_error(from: &str, error_code: c_int) -> DriverError {
    let mut alsa_desc: &str;
    unsafe {
        let cstr = CStr::from_ptr(snd_strerror(error_code));
        alsa_desc = cstr.to_str().unwrap();
    }
    return DriverError::new(error_code as i64, from, alsa_desc);
}


pub type SndSize = snd_pcm_uframes_t;


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
    pub fn new() -> Result<Params, DriverError> {
        let mut param_ptr: *mut snd_pcm_hw_params_t = ptr::null_mut();
        let mut err = 0;
        unsafe {
            let err = snd_pcm_hw_params_malloc(&mut param_ptr);
        }
        if err < 0 {
            return Err(create_error("snd_pcm_hw_params_malloc", err));
        }
        else {
            return Ok(Params { hw_params: param_ptr });
        }
    }


    pub fn free(&self) {
        unsafe {
            snd_pcm_hw_params_free(self.hw_params);
        }
    }


    pub fn buffer_size(&self) -> Result<SndSize, DriverError> {
        let mut size: snd_pcm_uframes_t = 0;
        let mut err = 0;
        unsafe {
            err = snd_pcm_hw_params_get_buffer_size(self.hw_params, &mut size);
        }
        if err < 0 {
            return Err(create_error("snd_pcm_hw_params_get_buffer_size", err));
        }
        else {
            println!("Buffer size: {}", size);
            return Ok(size);
        }
    }


    pub fn any(&self, pcm: *mut snd_pcm_t) -> Option<DriverError> {
        let mut err = 0;
        unsafe {
            err = snd_pcm_hw_params_any(pcm, self.hw_params);
        }
        if err < 0 {
            return Some(create_error("snd_pcm_hw_params_any", err));
        }
        else {
            return None;
        }
    }


    pub fn format(&self, channels: usize, pcm: *mut snd_pcm_t) -> Option<DriverError> {
        let mut err = 0;
        let mut rate: c_uint = 48000;
        unsafe {
            err = snd_pcm_hw_params_set_rate_resample(pcm, self.hw_params, 1);
            err = snd_pcm_hw_params_set_access(pcm, self.hw_params, SND_PCM_ACCESS_RW_INTERLEAVED);
            err = snd_pcm_hw_params_set_format(pcm, self.hw_params, SND_PCM_FORMAT_S16_LE);
            err = snd_pcm_hw_params_set_channels(pcm, self.hw_params, channels as c_uint);
            err = snd_pcm_hw_params_set_rate_near(pcm, self.hw_params, &mut rate, ptr::null_mut());
        }
        if err < 0 {
            return Some(create_error("snd_pcm_hw_params_any", err));
        }
        else {
            return None;
        }
    }



    pub fn apply(&self, pcm: *mut snd_pcm_t) -> Option<DriverError> {
        let mut err = 0;
        unsafe {
            err = snd_pcm_hw_params(pcm, self.hw_params);
        }
        if err < 0 {
            return Some(create_error("snd_pcm_hw_params", err));
        }
        else {
            return None;
        }
    }
}


pub struct Device {
    channels: usize,
    pcm: *mut snd_pcm_t,
}


impl Device {
    pub fn open(name: &str) -> Result<Device, DriverError> {
        let mut pcm_ptr: *mut snd_pcm_t = ptr::null_mut();
        let devname = CString::new(name).unwrap();
        let mut err = 0;
        let blocking = true;
        let flags = if blocking { 0 } else { SND_PCM_NONBLOCK };
        unsafe {
            err = snd_pcm_open(&mut pcm_ptr, devname.as_ptr(), SND_PCM_STREAM_PLAYBACK, flags);
        }
        if err < 0 {
            return Err(create_error("snd_pcm_open", err));
        }
        else {
            return Ok(Device { channels: 2, pcm: pcm_ptr });
        }
    }


    pub fn setup(&self, params: &mut Params) -> Option<DriverError> {
        match params.any(self.pcm) {
            Some(e) => return Some(e),
            None => {},
        }
        match params.format(self.channels, self.pcm) {
            Some(e) => return Some(e),
            None => {},
        }
        return params.apply(self.pcm);
    }


    pub fn prepare(&self) -> Option<DriverError> {
        let mut err = 0;
        unsafe {
            err = snd_pcm_prepare(self.pcm);
        }
        if err < 0 {
            return Some(create_error("snd_pcm_prepare", err));
        }
        else {
            return None;
        }
    }


    pub fn blocking(&self, block: bool) -> Result<i32, DriverError> {
        let mut result = 0;
        unsafe {
            if block {
                result = snd_pcm_nonblock(self.pcm, 0);
            }
            else {
                result = snd_pcm_nonblock(self.pcm, SND_PCM_NONBLOCK);
            }
        }
        if result < 0 {
            return Err(create_error("snd_pcm_nonblock", result));
        }
        else {
            return Ok(result);
        }
    }


    pub fn wait(&self) -> Result<i32, DriverError> {
        let mut result = 0;
        unsafe {
            result = snd_pcm_wait(self.pcm, -1);
        }
        if result < 0 {
            return Err(create_error("snd_pcm_wait", result));
        }
        else {
            return Ok(result);
        }
    }


    pub fn write_some(&self, data: &[i16]) -> Result<usize, DriverError> {
        let data_ptr = data.as_ptr() as *const c_void;
        let frames = (data.len() / self.channels) as snd_pcm_uframes_t;
        let mut size = 0;
        unsafe {
            match self.wait() {
                Ok(status) => size = snd_pcm_writei(self.pcm, data_ptr, frames),
                Err(err) => return Err(err),
            }
        }
        if size < 0 {
            return Err(create_error("snd_pcm_writei", size as i32));
        }
        else {
            return Ok(size as usize);
        }
    }


    pub fn output(&self, data: &[i16]) -> Result<SndSize, DriverError> {
        let available = data.len() / self.channels;
        let mut written: usize = 0;
        while written < available {
            let subdata = &data[written..data.len()];
            match self.write_some(subdata) {
                Ok(count) => written += count,
                Err(err) => return Err(err),
            }
        }
        return Ok(written as SndSize);
    }
}


impl AudioDriver for Device {
    fn init(&self) {
        match Params::new() {
            Ok(mut params) => {
                self.setup(&mut params);
                params.buffer_size();
                params.free();
                self.blocking(true);
                self.prepare();

            },
            Err(e) => println!("Param error: {}", e.as_string()),
        }
    }

    fn play(&self, data: &[i16]) {
        match self.output(&data) {
            Ok(size) => println!("Played {} samples", size),
            Err(e) => println!("Play error: {}", e.as_string()),
        }
    }
}
