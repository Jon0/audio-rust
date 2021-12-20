use format::sample::SampleType;
use sound::frame::Frame;
use sound::array::*;
use sound::sequence::*;

pub trait Generator {
	fn fill_async<F: SampleType>(&mut self, offset: usize, out: &mut [F]);
}


pub struct FrameGenerator {
	frame_samples: usize,
	frames: Vec<Frame>
}


impl FrameGenerator {
	pub fn new(samples_to_frame: usize) -> FrameGenerator {
		return FrameGenerator {frame_samples: samples_to_frame, frames: Vec::new()};
	}

	/**
	 * new generation function
	 */
	fn fill_frames(&mut self, size: usize) {
		for _block in self.frames.len()..(size + 1) {
			let new_frame = create_next_frame_v1(self.frames.len(), &self.frames);
			self.frames.push(new_frame);
		}
	}

	pub fn generating_function<F: SampleType>(&mut self, offset: usize, data: &mut [F]) {

		let sample_end = offset + data.len();
		let block_length =  self.frame_samples;
		let block_start: usize = offset / block_length;
		let block_end: usize = (sample_end / block_length) + 1;
		let amp = 1000.0;

		// add required frames
		self.fill_frames(block_end);

		// convert frames to samples
		for block in block_start..block_end {

			// start_time is the sample index of the blocks first sample
			let start_time = (block_length * block) as f64;
			let mut block_start_sample = block_length * block;
			let mut block_end_sample = (block_length * (block + 1)) - offset;

			// off is the position of data[0] relative to the start of the block
			let mut off = 0;

			if block_start_sample < offset {
				off = offset % block_length;
				block_start_sample = 0;
			}
			else {
				block_start_sample -= offset;
			}

			if block_end_sample >= data.len() {
				block_end_sample = data.len();
			}
			self.frames[block].fill(440.0, amp, 0.0, start_time, off, block_length, &mut data[block_start_sample..block_end_sample]);
			self.frames[block + 1].fill(440.0, 0.0, amp, start_time, off, block_length, &mut data[block_start_sample..block_end_sample]);
		}

	}
}


impl Generator for FrameGenerator {
	fn fill_async<F: SampleType>(&mut self, offset: usize, out_data: &mut [F]) {
		self.generating_function(offset, out_data);
	}
}
