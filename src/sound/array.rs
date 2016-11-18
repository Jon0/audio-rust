use std::io;
use std::f32;


pub trait Seq<T> {
    fn range(&self, start: usize, end: usize) -> &[T];
    fn inverse(&self, &T) -> Option<usize>;
}


pub struct PrimeSeq {
    values: Vec<u64>,
    max: u64,
}


impl PrimeSeq {
    pub fn new() -> PrimeSeq {
        PrimeSeq { values: vec![], max: 2 }
    }
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


pub fn isqrt(number: u64) -> u64 {
    return (number as f64).sqrt().ceil() as u64;
}


pub fn find_primes(ps: &mut PrimeSeq, max: u64) {
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


pub fn sequence(seq: &Seq<u64>, start: u32, size: usize) {
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



pub fn fill_sine(data: &mut [i16]) {
    for t in 0..data.len() {
        let fq = (t as f32) * 0.03;
        let x = fq.sin() * 2500.0;
        data[t] = x as i16;
    }
}


pub fn fill_bits(data: &mut [i16]) {
    for t in 0..data.len() {
        let ts = ((t as f32) * 0.1) as i16;
        let val = (ts | (ts >> 11 | ts >> 7)).wrapping_mul(ts & (ts >> 13 | ts >> 11));
        data[t] = val;
    }
}
