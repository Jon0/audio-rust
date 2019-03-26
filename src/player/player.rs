use std::thread;
use std::sync::mpsc;

use sound::generator::Generator;
use format::error::*;
use format::sample::SampleType;
use format::sample::Stream;


pub trait AudioDriver {
	fn init(&self);
	fn play(&self, data: &[i16]);
}

pub struct AudioPlayer {

}

impl AudioPlayer {
	pub fn new() -> AudioPlayer {
		return AudioPlayer {};
	}

	pub fn run<F: Clone + SampleType, S: Stream<F>, G: Generator>(&self, stream: &mut S, gen: &mut G) {

		let mut out = vec![F::zero(); 1024 * 64];
		let mut offset = 0;

		loop {
			for i in 0..out.len() {
				out[i] = F::zero();
			}

			gen.fill_async(offset, &mut out);
			stream.push(&out);
			offset += out.len();
		}
	}
}
