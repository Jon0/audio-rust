extern crate libc;


mod device {
    mod alsa;
    pub mod mixer;
}


use std::io;


use device::mixer::Mixer;


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


fn main() {
    let data = vec![0; 1024];
    let mut mux = Mixer::new();
    Mixer::open(&mut mux);
    Mixer::set_params(&mut mux);
    Mixer::prepare(&mut mux);
    Mixer::play(&mut mux, &data);


    let mut primes = PrimeSeq { values: vec![], max: 2 };
    find_primes(&mut primes, 2000);
    sequence(&primes, asknumber(), 8);
}
