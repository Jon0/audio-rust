use std::f64::consts::PI;
use num_rational::Rational64;

use sound::array::*;
use sound::number::*;


/**
 * replacement for Frame
 */
pub struct Frame {
    denom: Factorised<u64>,
    components: Vec<(Factorised<u64>, f64)>
}


impl Frame {
    pub fn new() -> Frame {
        return Frame {denom: Factorised::new(), components: Vec::new()};
    }

    pub fn from_pair(a: u64, b: u64) -> Frame {
        let mut init = Frame::new();

        let a_fct = factors(a);
        let b_fct = factors(b);
        let amp = 1.0 / ((a_fct.len() * b_fct.len()) as f64);
        for x in &a_fct {
            for y in &b_fct {
                init.push(*x, *y, amp);
            }
        }
        return init;
    }

    pub fn push(&mut self, numer: u64, denom: u64, amp: f64) {
        self.denom.to_union(denom);
        self.components.push((Factorised::create(numer), amp));
    }

    pub fn push_ratio(&mut self, val: Rational64, amp: f64) {
        self.push(*val.numer() as u64, *val.denom() as u64, amp);
    }

    pub fn gen_sample(&self, base: f64, time: f64) -> f64 {
        let sample_rate = 44100.0;
        let mut out = 0.0;

        for (component, c_amp) in &self.components {
            let freq = base * (component.val() as f64 / self.denom.val() as f64);
            let sample_freq = (2.0 * PI * freq) / sample_rate;
            let fq = time * sample_freq;
            let amp_adjust = 20.0 / freq.sqrt();
            out += fq.sin() * c_amp * amp_adjust;
        }
        return out;
    }

    pub fn fill(&self, base: f64, start_amp: f64, end_amp: f64, start_time: f64, offset: usize, frame_samples: usize, out: &mut [f64]) {
        for sample in 0..out.len() {
            let real_sample = sample + offset;
            let percent = real_sample as f64 / frame_samples as f64;
            let amp = percent * end_amp + (1.0 - percent) * start_amp;
            let s = start_time + real_sample as f64;

            out[sample] += self.gen_sample(base, s) * amp;
        }
    }
}



pub struct FrameOld {
    numer_product: u64,
    denom_product: u64,
    numer_product_factors: Vec<u64>,
    denom_product_factors: Vec<u64>,
    components: Vec<(Rational64, f64)>
}


impl FrameOld {
    pub fn create(a: u64, b: u64) -> FrameOld {
        let sum = Rational64::new(a as i64, b as i64);
        let a_fct = factors(a);
        let b_fct = factors(b);
        let amp = 1.0 / ((a_fct.len() * b_fct.len()) as f64);
        let mut vec = Vec::new();

        for x in &a_fct {
            for y in &b_fct {
                vec.push((Rational64::new(*x as i64, *y as i64), amp));
            }
        }

        FrameOld {
            numer_product: a,
            denom_product: b,
            numer_product_factors: a_fct,
            denom_product_factors: b_fct,
            components: vec
        }
    }


    pub fn create_next(&self, a: u64, b: u64) -> FrameOld {

        let sum = Rational64::new(a as i64, b as i64);
        let a_fct = factors(a);
        let b_fct = factors(b);
        let amp = 1.0 / ((a_fct.len() * b_fct.len()) as f64);

        let (numer_common, numer_uncommon) = common_factors(&self.numer_product_factors, &a_fct);
        let (denom_common, denom_uncommon) = common_factors(&self.denom_product_factors, &b_fct);
        println!("In {:?} / {:?}", a_fct, b_fct);
        println!("Common {:?} / {:?}", numer_common, denom_common);
        println!("Uncommon {:?} / {:?}", numer_uncommon, denom_uncommon);

        let numer_fct = a_fct;
        let denom_fct = b_fct;
        println!("Out {:?} / {:?}", numer_fct, denom_fct);
        let amp = 1.0 / ((numer_fct.len() * denom_fct.len()) as f64);
        let mut vec = Vec::new();

        for x in &numer_fct {
            for y in &denom_fct {
                vec.push((Rational64::new(*x as i64, *y as i64), amp));
            }
        }

        FrameOld {
            numer_product: product(&numer_fct),
            denom_product: product(&denom_fct),
            numer_product_factors: numer_fct,
            denom_product_factors: denom_fct,
            components: vec
        }
    }


    pub fn create_from_sequence(frames: &[FrameOld]) -> FrameOld {
        let mut numer_commons = Vec::new();
        let mut denom_commons = Vec::new();

        for frame in frames {
            let next_a = frame.numer_product + 1;
            let next_b = frame.denom_product - 1;

            let a_fct = factors(next_a);
            let b_fct = factors(next_b);

            numer_commons.push(a_fct);
            denom_commons.push(b_fct);
        }

        let new_a = high_freq_factors(&numer_commons, 2, 4);
        let new_b = high_freq_factors(&denom_commons, 2, 4);
        let amp = 1.0 / ((new_a.len() * new_b.len()) as f64);
        let mut vec = Vec::new();

        for x in &new_a {
            for y in &new_b {
                vec.push((Rational64::new(*x as i64, *y as i64), amp));
            }
        }

        FrameOld {
            numer_product: product(&new_a),
            denom_product: product(&new_b),
            numer_product_factors: new_a,
            denom_product_factors: new_b,
            components: vec
        }
    }


    pub fn contains(&self, item: Rational64) -> bool {
        let mut contains = false;
        for &(comp, amp) in &self.components {
            if (comp.numer() == item.numer() && comp.denom() == item.denom()) {
                contains = true;
            }
        }
        return contains;
    }


    pub fn print_freqs(&self, base: f64) {
        let mut out = Vec::new();

        for &(component, c_amp) in &self.components {
            let freq = base * (*component.numer() as f64 / *component.denom() as f64);
            out.push(freq);
        }

        println!("{:?}", out);
    }


    pub fn print_factors(&self) {
        println!("{:?} / {:?}", self.numer_product_factors, self.denom_product_factors);
    }

    pub fn output(a: Frame, b: Frame, base: f64, inter: f64) {

    }


}
