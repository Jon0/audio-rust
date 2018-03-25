use num_rational::Rational64;

use sound::array::*;


pub struct Frame {
    numer_product_factors: Vec<u64>,
    denom_product_factors: Vec<u64>,
    components: Vec<(Rational64, f64)>
}


impl Frame {
    pub fn create(a: u64, b: u64) -> Frame {
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

        Frame {
            numer_product_factors: a_fct,
            denom_product_factors: b_fct,
            components: vec
        }
    }


    pub fn create_next(&self, a: u64, b: u64) -> Frame {

        let sum = Rational64::new(a as i64, b as i64);
        let a_fct = factors(a);
        let b_fct = factors(b);
        let amp = 1.0 / ((a_fct.len() * b_fct.len()) as f64);

        let (numer_common, numer_uncommon) = common_factors(&self.numer_product_factors, &a_fct);
        let (denom_common, denom_uncommon) = common_factors(&self.denom_product_factors, &b_fct);
        println!("In {:?} / {:?}", a_fct, b_fct);
        println!("Common {:?} / {:?}", numer_common, denom_common);
        println!("Uncommon {:?} / {:?}", numer_uncommon, denom_uncommon);

        let numer_fct = numer_uncommon;
        let denom_fct = denom_common;
        println!("Out {:?} / {:?}", numer_fct, denom_fct);
        let amp = 1.0 / ((numer_fct.len() * denom_fct.len()) as f64);
        let mut vec = Vec::new();

        for x in &numer_fct {
            for y in &denom_fct {
                vec.push((Rational64::new(*x as i64, *y as i64), amp));
            }
        }

        Frame {
            numer_product_factors: numer_fct,
            denom_product_factors: denom_fct,
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


    pub fn gen_sample(&self, base: f64, time: f64) -> f64 {
        let sample_rate = 44100.0;
        let mut out = 0.0;

        for &(component, c_amp) in &self.components {
            let freq = base * (*component.numer() as f64 / *component.denom() as f64);
            let sample_freq = freq / sample_rate;
            let fq = time * sample_freq;
            out += fq.sin() * c_amp;
        }
        return out;
    }


    pub fn fill(&self, base: f64, start_amp: f64, end_amp: f64, start_time: f64, out: &mut [f64]) {
        for sample in 0..out.len() {
            let percent = sample as f64 / out.len() as f64;
            let amp = percent * end_amp + (1.0 - percent) * start_amp;
            let s = start_time + sample as f64;

            out[sample] += self.gen_sample(base, s) * amp;
        }
    }

}


pub fn output(a: Frame, b: Frame, base: f64, inter: f64) {

}
