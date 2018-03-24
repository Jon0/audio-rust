use num_rational::Rational64;

use sound::array::*;


pub struct Frame {
    components: Vec<(Rational64, f64)>
}


impl Frame {
    pub fn from_u64(value: u64) -> Frame {
        let fct = factors(value);


        Frame { components: fct }
    }







}


pub fn output(a: Frame, b: Frame, base: f64, inter: f64) {
    
}
