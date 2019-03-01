extern crate libc;
extern crate rand;
extern crate num_integer;
extern crate num_rational;
extern crate audio_rust;

use std::io;
use std::env;
use audio_rust::player::player::*;
use audio_rust::alsa::mixer::*;
use audio_rust::sound::array::*;
use audio_rust::sound::generator::*;
use audio_rust::sound::sampler::*;


/**
 * Moved to AudioDriver
 */
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
    let mut generator = FrameGenerator::new(1024 * 64);
    let mut buffer = vec![0.0; 1024 * 1024 * 24];
    let mut out = vec![0; 1024 * 1024 * 24];
    //sample_function(test_fn, &mut buffer);
    generator.generating_function(0, &mut buffer);
    data_to_i16(&mut out, &buffer);

    println!("playing...");

    match dev.output(&out) {
        Ok(size) => println!("Played {} samples", size),
        Err(e) => println!("Play error: {}", e.as_string()),
    }

}


fn use_device(mut dev: Device) {
    let mut generator = FrameGenerator::new(1024 * 16);
    let player = AudioPlayer::new();

    player.run(&mut dev, &mut generator);

    // wait for completion
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("failed to read line");
}


fn main() {
    let args: Vec<String> = env::args().collect();

    match Device::open(&args[1]) {
        Ok(mut dev) => {
            use_device(dev)
        },
        Err(err) => println!("Open error: {}", err.as_string()),
    }
}
