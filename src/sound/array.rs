extern crate rand;

use std::io;
use std::f32;
use std::f64;
use std::ops::Add;
use rand::{thread_rng, Rng};


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


pub fn factors(number: u64) -> Vec<u64> {
    let mut result = vec![];
    let mut remain = number;
    let mut i = 2;
    while (i <= remain) {
        if remain % i == 0 {
            remain /= i;
            result.push(i);
        }
        else {
            i += 1;
        }
    }
    return result;
}


/**
 * a and b must be sorted arrays
 * returns tuple of common and non-common factors
 */
pub fn common_factors(a: &Vec<u64>, b: &Vec<u64>) -> (Vec<u64>, Vec<u64>) {
    let mut common = Vec::new();
    let mut inverse = Vec::new();
    let mut a_index = 0;
    let mut b_index = 0;
    let max_len = if a.len() > b.len() { a.len() } else { b.len() };

    while a_index < a.len() && b_index < b.len() {
        let a_val = a[a_index];
        let b_val = b[b_index];

        if (a_val == b_val) {
            common.push(a_val);
            a_index += 1;
            b_index += 1;
        }
        else if (a_val < b_val) {
            inverse.push(a_val);
            a_index += 1;
        }
        else {
            inverse.push(b_val);
            b_index += 1;
        }
    }
    for a_remain in a_index..a.len() {
        inverse.push(a[a_remain]);
    }
    for b_remain in b_index..b.len() {
        inverse.push(b[b_remain]);
    }
    return (common, inverse);
}


pub fn is_prime(number: u64) -> bool {
    for i in 2..isqrt(number) {
        if number % i == 0 {
            return false;
        }
    }
    return true;
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


pub fn sum(numbers: Vec<u64>) -> u64 {
    let mut total = 0;
    for i in numbers {
        total += i;
    }
    return total;
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
        data[t] = val as i16;
    }
}


pub fn fill_wave(start: usize, end: usize, max: usize, data: &mut [f64]) {
    let mul = (f64::consts::PI * 2.0) / (data.len() as f64);
    let vol = (data.len() as f64) * 0.05;
    for t in 0..data.len() {
        let fq = (t as f64) * mul * 1000.0;
        let x = fq.sin() * vol;
        data[t] += x as f64;
    }
}


pub fn fill_test(start: usize, end: usize, max: usize, data: &mut [f64]) {
    let smodulo = (max % start) as f64;
    let sdivide = (max / start) as f64;
    let emodulo = (max % end) as f64;
    let edivide = (max / end) as f64;
    let mul = (f64::consts::PI * 2.0) / (data.len() as f64);
    let vol = (((emodulo / edivide) * (smodulo / sdivide)).log(2.0) - 20.0) * 40.0;
    println!("vol {}", vol);
    for t in 0..data.len() {
        let fq = (t as f64) * mul * 200.0;
        let x = fq.sin() * vol;
        data[t] += x as f64;
    }
}


pub fn fill_with(filler: fn(usize, usize, usize, data: &mut [f64]), data: &mut [f64], samples: usize) {
    let sample_min = 30000;
    let sample_max = 1000000;
    let mut rng = rand::thread_rng();
    for s in 0..samples {
        println!("filling {}/{}", s, samples);
        let start = rng.gen_range(0, data.len());
        let end = rng.gen_range(start, data.len());
        let length = end - start;
        if sample_min < length && length < sample_max {
            filler(start, end, data.len(), &mut data[start..end]);
        }
    }
}


pub fn data_to_i16(out_data: &mut [i16], in_data: &[f64]) {
    for t in 0..in_data.len() {
        out_data[t] = in_data[t] as i16;
    }
}
