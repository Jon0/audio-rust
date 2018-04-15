extern crate libc;
extern crate rand;
extern crate num_integer;
extern crate num_rational;

mod device;
mod player;
mod sound;

use std::io;
use device::mixer::*;
use sound::array::*;
use sound::sampler::*;


fn init_audio(dev: &mut Device) {
    match Params::new() {
        Ok(mut params) => {
            dev.setup(&mut params);
            params.buffer_size();
            params.free();
            dev.blocking(true);
            dev.prepare();

        },
        Err(e) => println!("Param error: {}", e.as_string()),
    }
}


fn play_test(dev: &mut Device) {
    let mut buffer = vec![0.0; 1024 * 1024 * 24];
    let mut out = vec![0; 1024 * 1024 * 24];
    //sample_function(test_fn, &mut buffer);
    generating_function(&mut buffer);
    data_to_i16(&mut out, &buffer);

    println!("playing...");

    match Device::play(&dev, &out) {
        Ok(size) => println!("Played {} samples", size),
        Err(e) => println!("Play error: {}", e.as_string()),
    }

}


fn use_device(mut dev: &mut Device) {
    init_audio(&mut dev);
    play_test(&mut dev);

    // wait for completion
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("failed to read line");
}


fn main() {
    match Device::open("hw:0,0") {
        Ok(mut dev) => {
            use_device(&mut dev)
        },
        Err(err) => println!("Open error: {}", err.as_string()),
    }
}
