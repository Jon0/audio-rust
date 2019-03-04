use std::ptr;
use libc::c_int;
use libc::c_uint;
use libc::c_void;

use format::format::Format;
use format::sample::SampleType;
use alsa::format::AlsaFormat;
use alsa::mixer::Device;
use alsa::mixer::create_error;
use alsa::ffi::*;
use format::error::*;


pub struct Stream<S: SampleType> {

	device: Device,
	sample_rate: usize,
	buffer: Vec<S>

}


impl<F: AlsaFormat, S: SampleType<Sample=F>> Stream<S> {
	pub fn open(device: Device) -> Result<Self, DriverError> {

		let mut err = 0;
		let mut param_ptr: *mut snd_pcm_hw_params_t = ptr::null_mut();
		let mut rate: c_uint = 48000;
		
		unsafe {
			err = snd_pcm_hw_params_malloc(&mut param_ptr);
			err = snd_pcm_hw_params_set_rate_resample(device.get_pcm(), param_ptr, 1);
			err = snd_pcm_hw_params_set_access(device.get_pcm(), param_ptr, SND_PCM_ACCESS_RW_INTERLEAVED);
			err = snd_pcm_hw_params_set_format(device.get_pcm(), param_ptr, F::FormatId);
			err = snd_pcm_hw_params_set_channels(device.get_pcm(), param_ptr, S::Channels as c_uint);
			err = snd_pcm_hw_params_set_rate_near(device.get_pcm(), param_ptr, &mut rate, ptr::null_mut());
		}
		if err < 0 {
			return Err(create_error("snd_pcm_hw_params_malloc", err));
		}
		else {
			return Ok(Stream { device: device, sample_rate: 48000, buffer: Vec::new() });
		}	

	}

}