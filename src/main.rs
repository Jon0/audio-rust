extern crate libc;


mod device {
    mod alsa;
    pub mod format;
    pub mod mixer;
}

mod sound {
    pub mod array;
}

use std::io;

use device::mixer::*;
use sound::array::*;



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
            Device::setup(&dev, &mut params);
            Device::prepare(&dev);
        },
        Err(e) => println!("Param error: {}", e.as_string()),
    }
}






fn play_test(dev: &mut Device) {
    let mut data = vec![0; 1024 * 1024 * 32];
    fill_bits(&mut data);
    match Device::play(&dev, &data) {
        Ok(size) => println!("Played {} samples", size),
        Err(e) => println!("Play error: {}", e.as_string()),
    }
}


fn main() {
    match Device::open("plughw:0,0") {
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
