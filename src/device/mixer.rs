use std::ptr;
use std::ffi::CStr;
use std::ffi::CString;
use libc::c_int;
use libc::c_void;

use device::alsa::*;


pub struct Mixer {
    pcm: *mut snd_pcm_t,
    size: snd_pcm_uframes_t,
}


impl Mixer {
    pub fn new() -> Mixer {
        Mixer {
            pcm: ptr::null_mut(),
            size: 0,
        }
    }


    pub fn open(mixer: &mut Mixer) {
        let devname = CString::new("plughw:0,0").unwrap();
        unsafe {
            let err = snd_pcm_open(&mut mixer.pcm, devname.as_ptr(), SND_PCM_STREAM_PLAYBACK, SND_PCM_NONBLOCK);
            println!("snd_pcm_open: {}", error_string(err));

        }
    }


    pub fn set_params(mixer: &mut Mixer) {
        unsafe {
            let mut hw_params: *mut snd_pcm_hw_params_t = ptr::null_mut();
            let err1 = snd_pcm_hw_params_malloc(&mut hw_params);
            println!("snd_pcm_hw_params_malloc: {}", error_string(err1));
            let err2 = snd_pcm_hw_params_any(mixer.pcm, hw_params);
            println!("snd_pcm_hw_params_any: {}", error_string(err2));
            let err3 = snd_pcm_hw_params(mixer.pcm, hw_params);
            println!("snd_pcm_hw_params: {}", error_string(err3));
            let err4 = snd_pcm_hw_params_get_buffer_size(hw_params, &mut mixer.size);
            println!("snd_pcm_hw_params_get_buffer_size: {}, {}", mixer.size, error_string(err4));
        }
    }


    pub fn prepare(mixer: &mut Mixer) {
        unsafe {
            let err1 = snd_pcm_nonblock(mixer.pcm, SND_PCM_NONBLOCK);
            println!("snd_pcm_nonblock: {}", error_string(err1));
            let err2 = snd_pcm_prepare(mixer.pcm);
            println!("snd_pcm_prepare: {}", error_string(err2));
        }
    }


    pub fn play(mixer: &mut Mixer, data: &[i16]) {
        let available: snd_pcm_uframes_t = 1024;
        unsafe {
            let err1 = snd_pcm_wait(mixer.pcm, -1);
            println!("snd_pcm_wait: {}", error_string(err1));
            let size = snd_pcm_writei(mixer.pcm, data.as_ptr() as *const c_void, available);
            println!("snd_pcm_writei: {}", size);
        }
    }
}


fn error_string(err: c_int) -> String {
    unsafe {
        return CStr::from_ptr(snd_strerror(err)).to_string_lossy().into_owned();
    }
}
