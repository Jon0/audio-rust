use std::ptr;
use std::ffi::CStr;
use std::ffi::CString;
use libc::c_int;
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
            return Ok(size);
        }
    }
}


pub struct Device {
    pcm: *mut snd_pcm_t,
}


impl Device {
    pub fn open(name: &str) -> Result<Device, SndError> {
        let pcm_ptr: *mut snd_pcm_t = ptr::null_mut();
        let devname = CString::new(name).unwrap();
        let mut err = 0;
        unsafe {
            err = snd_pcm_open(&mut pcm_ptr, devname.as_ptr(), SND_PCM_STREAM_PLAYBACK, SND_PCM_NONBLOCK);
        }
        if err < 0 {
            return Err(SndError::new("snd_pcm_open", err));
        }
        else {
            return Ok(Device { pcm: pcm_ptr });
        }
    }


    pub fn config(&self) -> Result<Config, SndError> {

    }


    pub fn prepare(&self) -> Result<Mixer, SndError> {
        let mut err = 0;
        unsafe {
            err = snd_pcm_nonblock(self.pcm, SND_PCM_NONBLOCK);
        }
        if err < 0 {
            return Err(SndError::new("snd_pcm_nonblock", err));
        }

        unsafe {
            err = snd_pcm_prepare(self.pcm);
        }
        if err < 0 {
            return Err(SndError::new("snd_pcm_prepare", err));
        }
        else {
            return Ok(Mixer { dev: self, size: 0 })
        }
    }
}


pub struct Config {
    dev: Device,
    params: Params,
}


impl Config {
    pub fn new(d: Device, p: Params) -> Result<Config, SndError> {
        let mut err = 0;
        unsafe {
            err = snd_pcm_hw_params_any(d.pcm, p.hw_params);
        }
        if err < 0 {
            return Err(SndError::new("snd_pcm_hw_params_any", err));
        }
        else {
            return Ok(Config { dev: d, params: p });
        }
    }


    pub fn apply(&self) {
        let mut err = 0;
        unsafe {
            err = snd_pcm_hw_params(self.dev.pcm, self.params.hw_params);
        }
    }
}


pub struct Mixer {
    dev: Device,
    size: SndSize,
}


impl Mixer {
    pub fn play(mixer: &mut Mixer, data: &[i16]) -> Result<SndSize, SndError> {
        let available: SndSize = 1024;
        let mut size: SndSize = 0;
        let mut err = 0;
        unsafe {
            err = snd_pcm_wait(mixer.pcm, -1);
        }
        if err < 0 {
            return Err(SndError::new("snd_pcm_wait", err));
        }

        unsafe {
            size = snd_pcm_writei(mixer.pcm, data.as_ptr() as *const c_void, available);
        }
        if size < 0 {
            return Err(SndError::new("snd_pcm_writei", err));
        }
        else {
            return Ok(size);
        }
    }
}
