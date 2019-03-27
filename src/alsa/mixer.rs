use std::ptr;
use std::ffi::CStr;
use std::ffi::CString;
use std::cmp::min;
use libc::c_int;
use libc::c_uint;
use libc::c_void;

use format::error::DriverError;
use alsa::device::AlsaDevice;
use alsa::device::create_error;
use alsa::ffi::*;

pub type SndSize = snd_pcm_uframes_t;


pub struct Params {
	hw_params: *mut snd_pcm_hw_params_t,
}


impl Params {
	pub fn new() -> Result<Params, DriverError> {
		let mut param_ptr: *mut snd_pcm_hw_params_t = ptr::null_mut();
		let mut err = 0;
		unsafe {
			err = snd_pcm_hw_params_malloc(&mut param_ptr);
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


	pub fn any(&self, dev: &AlsaDevice) -> Option<DriverError> {
		
		let pcm = dev.get_pcm();
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


	pub fn format(&self, dev: &AlsaDevice, rate_in: c_uint, channels: c_uint, format: c_int) -> Option<DriverError> {
		
		let pcm = dev.get_pcm();
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



	pub fn apply(&self, dev: &AlsaDevice) -> Option<DriverError> {

		let pcm = dev.get_pcm();
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
