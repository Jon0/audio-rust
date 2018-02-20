extern crate libc;
extern crate rand;

mod device;
mod sound;

use std::io;
use device::mixer::*;
use sound::array::*;
use sound::sampler::*;


fn printc(text: String, colour: u32) {
    println!("\x1b[1;{};1m{}", colour, text);
}


fn asknumber() -> u32 {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).expect("failed to read line");
    let number: u32 = match buf.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    return number;
}


fn init_audio(dev: &mut Device) {
    match Params::new() {
        Ok(mut params) => {
            dev.setup(&mut params);
            params.free();
            dev.blocking(true);
            dev.prepare();
        },
        Err(e) => println!("Param error: {}", e.as_string()),
    }
}


fn play_test(dev: &mut Device) {
    let mut buffer = vec![0.0; 1024 * 1024];
    let mut out = vec![0; 1024 * 1024];
    sample_function(test_fn, &mut buffer);
    data_to_i16(&mut out, &buffer);
    println!("playing...");
    match Device::play(&dev, &out) {
        Ok(size) => println!("Played {} samples", size),
        Err(e) => println!("Play error: {}", e.as_string()),
    }
}


fn number_test() {
    for i in 1..100 {
        println!("{} has factors {:?} sum is {}", i, factors(i), sum(factors(i)));
    }
}


fn main() {
    number_test();

    match Device::open("hw:0,0") {
        Ok(mut d) => {
            init_audio(&mut d);
            play_test(&mut d);
        },
        Err(e) => println!("Open error: {}", e.as_string()),
    }


    let mut primes = PrimeSeq::new();
    find_primes(&mut primes, 2000);
    sequence(&primes, asknumber(), 8);
}
