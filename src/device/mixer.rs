use std::ptr;
use std::ffi::CStr;
use std::ffi::CString;

use device::alsa::*;


pub struct Mixer {
    pcm: *mut snd_pcm_t,
}


impl Mixer {
    pub fn new() -> Mixer {
        Mixer {
            pcm: ptr::null_mut(),
        }
    }


    pub fn open(mixer: &mut Mixer) {
        let devname = CString::new("plughw:0,0").unwrap();
        unsafe {
            let err = snd_pcm_open(&mut mixer.pcm, devname.as_ptr(), SND_PCM_STREAM_PLAYBACK, SND_PCM_NONBLOCK);
            println!("snd_pcm_open: {}", err);
            let preperr = snd_pcm_prepare(mixer.pcm);
            let errstr = CStr::from_ptr(snd_strerror(preperr)).to_string_lossy().into_owned();
            println!("snd_pcm_prepare: {}", errstr);
        }
    }
}
