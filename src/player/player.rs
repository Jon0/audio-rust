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
        let mut out = vec![0; 1024 * 1024];
        let mut offset = 0;

        driver.init();
        loop {
            println!("Filling buffer");
            gen.fill_async(offset, &mut out);
            driver.play(&out);
            offset += 1024 * 1024;
        }
    }
}
