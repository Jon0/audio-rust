use std::thread;
use std::sync::mpsc;

use sound::generator::Generator;

#[derive(Debug)]
pub struct DriverError {
    err: i64,
    name: String,
    desc: String,
}

impl DriverError {
    pub fn new(e: i64, n: &str, d: &str) -> DriverError {
        DriverError {  err: e, name: String::from(n), desc: String::from(d) }
    }

    pub fn as_string(&self) -> String {
        return self.desc.clone();
    }
}

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

    pub fn run<D: AudioDriver, G: Generator>(&self, driver: &mut D, gen: &mut G) {
        let mut out = vec![0.0; 1024 * 64];
        let mut buffer = vec![0; 1024 * 64];
        let mut offset = 0;

        driver.init();
        loop {
            gen.fill_async(offset, &mut out);

            for i in 0..out.len() {
                buffer[i] = out[i] as i16;
            }

            driver.play(&buffer);
            offset += out.len();
        }
    }
}
