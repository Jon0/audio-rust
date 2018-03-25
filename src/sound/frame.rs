use num_rational::Rational64;

use sound::array::*;


pub struct Frame {
    components: Vec<(Rational64, f64)>
}


impl Frame {
    pub fn create(a: u64, b: u64) -> Frame {
        let a_fct = factors(a);
        let b_fct = factors(b);
        let mut vec = Vec::new();

        for x in &a_fct {
            for y in &b_fct {
                vec.push((Rational64::new(*x as i64, *y as i64), 1.0));
            }
        }

        Frame { components: vec }
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


    pub fn fill(&self, base: f64, amp: f64, out: &mut [f64]) {
        for sample in 0..out.len() {
            out[sample] += self.gen_sample(base, sample as f64) * amp;
        }
    }

}


pub fn output(a: Frame, b: Frame, base: f64, inter: f64) {

}
