extern crate libc;


mod device {
    mod alsa;
    pub mod format;
    pub mod mixer;
}


use std::io;
use std::f32;

use device::mixer::*;


trait Seq<T> {
    fn range(&self, start: usize, end: usize) -> &[T];
    fn inverse(&self, &T) -> Option<usize>;
}


struct PrimeSeq {
    values: Vec<u64>,
    max: u64,
}


impl Seq<u64> for PrimeSeq {
    fn range(&self, start: usize, end: usize) -> &[u64] {
        return &self.values[start..end];
    }


    fn inverse(&self, elem: &u64) -> Option<usize> {
        return match self.values.binary_search(elem) {
            Ok(index) => Some(index),
            Err(_) => None,
        }
    }
}


fn isqrt(number: u64) -> u64 {
    return (number as f64).sqrt().ceil() as u64;
}


fn find_primes(ps: &mut PrimeSeq, max: u64) {
    let mut number = ps.max;
    while number < max {
        let mut isprime = true;
        for i in 2..isqrt(number) {
            if number % i == 0 {
                isprime = false;
            }
        }
        if isprime {
            ps.values.push(number);
        }
        number += 1;
    }
    ps.max = max;
}


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


fn sequence(seq: &Seq<u64>, start: u32, size: usize) {
    let mut grid = vec![0; size * size];
    let start: usize = start as usize;
    for i in 0..size*size {
        let number = start + i;
        grid[i] = number;
    }
    for i in 0..size {
        for j in 0..size {
            print!("{}, ", grid[i * size + j])
        }
        println!("");
    }
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



fn fill_sine(data: &mut [i16]) {
    for t in (0..data.len()) {
        let fq = (t as f32) * 0.03;
        let x = fq.sin() * 2500.0;
        data[t] = x as i16;
    }
}


fn fill_bits(data: &mut [i16]) {
    for t in (0..data.len()) {
        let ts = ((t as f32) * 0.05) as i16;
        let val = (ts | (ts >> 11 | ts >> 7)).wrapping_mul(ts & (ts >> 13 | ts >> 11));
        data[t] = val;
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


    let mut primes = PrimeSeq { values: vec![], max: 2 };
    find_primes(&mut primes, 2000);
    sequence(&primes, asknumber(), 8);
}
