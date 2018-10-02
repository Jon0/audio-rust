use sound::frame::Frame;
use sound::array::*;
use sound::sequence::*;

pub trait Generator {
    fn fill_async(&mut self, offset: usize, out: &mut [i16]);
}


pub struct FrameGenerator {
    frame_samples: usize,
    frames: Vec<Frame>
}


impl FrameGenerator {
    pub fn new() -> FrameGenerator {
        return FrameGenerator {frame_samples: 1024, frames: Vec::new()};
    }

    /**
     * the old generating function
     */
    fn create_frame(&mut self) {
        // initial frames
        if self.frames.len() == 0 {
            for frame in 0..12 {
                let x = frame as u64;
                let n = 41 + x;
                let d = 32 - x;
                self.frames.push(Frame::create(n, d));
            }
        }

        let end = self.frames.len() - 1;
        let this_frame = Frame::create_from_sequence(&self.frames[end - 11 .. end]);
        this_frame.print_factors();
        self.frames.push(this_frame);
    }

    /**
     * new generation function
     */
    fn fill_frames(&mut self, size: usize) {
        for block in self.frames.len()..(size + 1) {
            println!("Block {}", block);
            let new_frame = create_next_frame_v1(&self.frames);
            self.frames.push(new_frame);
        }
    }

    pub fn generating_function(&mut self, offset: usize, data: &mut [f64]) {

        let sample_end = offset + data.len();
        let block_length =  self.frame_samples;
        let block_start: usize = offset / block_length;
        let block_end: usize = (sample_end / block_length) + 1;
        let amp = 1200.0;

        // add required frames
        self.fill_frames(block_end);

        // convert frames to samples
        for block in block_start..block_end {
            let start_time = (block_length * block) as f64;
            let mut block_start_sample = block_length * block;
            let mut block_end_sample = (block_length * (block + 1)) - offset;
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
    fn fill_async(&mut self, offset: usize, out_data: &mut [i16]) {

        let mut buffer = vec![0.0; out_data.len()];
        self.generating_function(offset, &mut buffer);

        for t in 0..buffer.len() {
            out_data[t] = buffer[t] as i16;
        }
    }
}
