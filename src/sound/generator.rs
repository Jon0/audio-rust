use sound::frame::Frame;
use sound::array::*;
use sound::sampler::*;

pub trait Generator {
    fn fill_async(&self, out: &mut [i16]);
}


pub struct FrameGenerator {
    frames: Vec<Frame>
}


impl FrameGenerator {
    pub fn new() -> FrameGenerator {
        return FrameGenerator {frames: Vec::new()};
    }
}


impl Generator for FrameGenerator {
    fn fill_async(&self, out_data: &mut [i16]) {

        let mut buffer = vec![0.0; 1024 * 1024 * 24];
        generating_function(&mut buffer);

        for t in 0..buffer.len() {
            out_data[t] = buffer[t] as i16;
        }
    }
}
