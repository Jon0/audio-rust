use std::io;
use std::f32;
use std::f64;

use sound::array::*;
use sound::frame::*;

pub fn create_freq_table(tuning: f64) -> [f64; 128] {
    let mut freq_table = [0.0; 128];
    let two: f64 = 2.0;
    let ratio = two.powf(1.0 / 12.0);
    for f in 0..128 {
        freq_table[f] = tuning * ratio.powf((f as f64) - 64.0);
    }
    return freq_table;
}


pub fn test_fn(time: usize, max: usize, freq: f64) -> f64 {
    let p = (time as f64) / (max as f64);
    return p * (1.0 - p) * 4.0;
}


pub fn apply_filler(filler: fn(usize, usize, f64) -> f64, time: usize, max: usize, freq_table: &[f64], freq_state: &mut [f64]) {
    for s in 0..freq_state.len() {
        freq_state[s] = filler(time, max, freq_table[s]);
    }
}


/* sample points */
pub fn sample_function(filler: fn(usize, usize, f64) -> f64, data: &mut [f64]) {
    let v = 1000.0;
    let sample_rate = 44100.0;
    let block_count = 1024;
    let block_length = data.len() / block_count as usize;
    let freq_table = create_freq_table(440.0);
    let mut freq_state_start = [0.0; 128];
    let mut freq_state_end = [0.0; 128];
    let mut sample_number = 0;
    //apply_filler(filler, 0, block_count, &freq_table, &mut freq_state_start);
    for block in 1..block_count {
        apply_filler(filler, block, block_count, &freq_table, &mut freq_state_end);
        for sample in 0..block_length {
            let inter_start = sample as f64 / block_length as f64;
            let inter_end = 1.0 - inter_start;
            for freq in 0..freq_table.len() {
                let mul = freq_table[freq] / sample_rate;
                let fq = (sample_number as f64) * mul;
                //println!("fq {}", mul);
                //println!("s {} {}, e {} {}", freq_state_start[freq], inter_start, freq_state_end[freq], inter_end);
                data[sample_number] += fq.sin() * (freq_state_start[freq] * inter_start + freq_state_end[freq] * inter_end) * v;
            }
            sample_number += 1;
        }
        freq_state_start = freq_state_end;
    }
}


pub fn gen_sequence(pre_state: &[u64], time: u64) {

    //

}



pub fn gen_from_previous(data: &[Frame]) -> Frame {
    let mut result  = [0.0; 128];


    return Frame::from_u64(0);
}


pub fn generating_function(data: &mut [f64]) {
    let v = 1000.0;
    let sample_rate = 44100.0;
    let block_count = 32;

    let block_length = data.len() / block_count as usize;

    let freq_table = create_freq_table(440.0);
    let mut freq_state_start = [0.0; 128];
    let mut freq_state_end = [0.0; 128];
    let mut sample_number = 0;

    let mut seq_1 = 1;
    let mut seq_2 = 2;
    let mut seq_3 = block_count;

    for block in 1..block_count {
        println!("Block {} => {}", block, seq_2);
        let f = factors(seq_2);
        println!("Factors {:?}", f);
        let m = 1.0 / (f.len() as f64);
        for activation in 0..freq_state_end.len() {
            freq_state_end[activation] *= 0.5;
            let index = activation % f.len();
            if f[index] == 2 {
                freq_state_end[activation] += 0.5;
            }
        }

        //apply_filler(filler, block, block_count, &freq_table, &mut freq_state_end);
        for sample in 0..block_length {
            let inter_start = sample as f64 / block_length as f64;
            let inter_end = 1.0 - inter_start;
            for freq in 0..freq_table.len() {
                let mul = freq_table[freq] / sample_rate;
                let fq = (sample_number as f64) * mul;

                data[sample_number] += fq.sin() * (freq_state_start[freq] * inter_start + freq_state_end[freq] * inter_end) * v;
            }
            sample_number += 1;
        }
        freq_state_start = freq_state_end;

        seq_2 = seq_1 + seq_2;
        seq_1 = seq_2 - seq_1;
        seq_3 = seq_3 - 1;
    }
}
