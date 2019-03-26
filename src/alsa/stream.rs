use std::ptr;
use libc::c_int;
use libc::c_uint;
use libc::c_void;

use format::format::Format;
use format::sample::SampleType;
use format::sample::Stream;
use alsa::format::AlsaFormat;
use alsa::mixer::Params;
use alsa::mixer::Device;
use alsa::mixer::create_error;
use alsa::ffi::*;
use format::error::*;


pub struct AlsaStream<S: SampleType> {

	device: Device,
	sample_rate: usize,
	buffer: Vec<S>

}


impl<F: AlsaFormat, S: SampleType<Sample=F>> AlsaStream<S> {
	pub fn open(device: Device) -> Result<Self, DriverError> {

		let sample_rate = 48000;
		let mut params = Params::new().expect("Failed to create hw params");

		params.any(&device);
		params.format(&device, sample_rate, S::Channels as c_uint, F::FormatId);
		params.apply(&device);

		//device.setup(&mut params);
		let buffer_size = params.buffer_size();
		params.free();

		return Ok(AlsaStream { device: device, sample_rate: sample_rate as usize, buffer: Vec::new() });
	}


	pub fn wait(&self) -> Result<i32, DriverError> {
		let mut result = 0;
		unsafe {
			result = snd_pcm_wait(self.device.get_pcm(), -1);
		}
		if result < 0 {
			return Err(create_error("snd_pcm_wait", result));
		}
		else {
			return Ok(result);
		}
	}

	pub fn write_some(&self, data: &[S]) -> Result<usize, DriverError> {

		let data_ptr = data.as_ptr() as *const c_void;
		let frames = data.len() as snd_pcm_uframes_t;
		let mut size = 0;
		unsafe {
			match self.wait() {
				Ok(status) => size = snd_pcm_writei(self.device.get_pcm(), data_ptr, frames),
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

	pub fn output(&self, data: &[S]) -> Result<usize, DriverError> {
		let available = data.len();
		let mut written: usize = 0;
		while written < available {
			let subdata = &data[written..data.len()];
			match self.write_some(subdata) {
				Ok(count) => written += count,
				Err(err) => return Err(err),
			}
		}
		return Ok(written);
	}
}


impl<F: AlsaFormat, S: SampleType<Sample=F>> Stream<S> for AlsaStream<S> {

	fn push(&mut self, frames: &[S]) {
		self.output(frames);
	}
}