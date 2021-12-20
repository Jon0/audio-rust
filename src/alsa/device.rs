use std::ptr;
use std::ffi::CStr;
use std::ffi::CString;
use std::cmp::min;
use libc::c_int;
use libc::c_uint;
use libc::c_void;

use format::error::DriverError;
use alsa::mixer::Params;
use alsa::ffi::*;

pub fn create_error(from: &str, error_code: c_int) -> DriverError {
	let alsa_desc: &str;
	unsafe {
		let cstr = CStr::from_ptr(snd_strerror(error_code));
		alsa_desc = cstr.to_str().unwrap();
	}
	return DriverError::new(error_code as i64, from, alsa_desc);
}

pub struct AlsaDevice {
	pcm: *mut snd_pcm_t,
}


impl AlsaDevice {
	pub fn open(name: &str) -> Result<Self, DriverError> {
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
			return Ok(AlsaDevice { pcm: pcm_ptr });
		}
	}


	pub fn get_pcm(&self) -> *mut snd_pcm_t {
		return self.pcm;
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
