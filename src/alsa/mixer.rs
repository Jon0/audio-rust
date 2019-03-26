use std::ptr;
use std::ffi::CStr;
use std::ffi::CString;
use std::cmp::min;
use libc::c_int;
use libc::c_uint;
use libc::c_void;


use alsa::ffi::*;
use format::error::*;
use player::player::*;



fn create_error_string(error_code: c_int) -> String {
    unsafe {
        return CStr::from_ptr(snd_strerror(error_code)).to_string_lossy().into_owned();
    }
}


pub fn create_error(from: &str, error_code: c_int) -> DriverError {
    let mut alsa_desc: &str;
    unsafe {
        let cstr = CStr::from_ptr(snd_strerror(error_code));
        alsa_desc = cstr.to_str().unwrap();
    }
    return DriverError::new(error_code as i64, from, alsa_desc);
}


pub type SndSize = snd_pcm_uframes_t;


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


    pub fn any(&self, dev: &Device) -> Option<DriverError> {
        
        let mut pcm = dev.get_pcm();
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


    pub fn format(&self, dev: &Device, rate_in: c_uint, channels: c_uint, format: c_int) -> Option<DriverError> {
        
        let mut pcm = dev.get_pcm();
        let mut err = 0;
        let mut rate: c_uint = rate_in;

        unsafe {
            err = snd_pcm_hw_params_set_rate_resample(pcm, self.hw_params, 1);
            err = snd_pcm_hw_params_set_access(pcm, self.hw_params, SND_PCM_ACCESS_RW_INTERLEAVED);
            err = snd_pcm_hw_params_set_format(pcm, self.hw_params, format);
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



    pub fn apply(&self, dev: &Device) -> Option<DriverError> {

        let mut pcm = dev.get_pcm();
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
            return Ok(Device { pcm: pcm_ptr });
        }
    }


    pub fn get_pcm(&self) -> *mut snd_pcm_t {
        return self.pcm;
    }

    pub fn setup(&self, params: &mut Params) -> Option<DriverError> {
        match params.any(self) {
            Some(e) => return Some(e),
            None => {},
        }
        match params.format(self, 48000, 2, SND_PCM_FORMAT_S16_LE) {
            Some(e) => return Some(e),
            None => {},
        }
        return params.apply(self);
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

}
