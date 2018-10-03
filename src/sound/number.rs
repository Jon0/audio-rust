use num_integer::Integer;


pub struct Factorised<T> {
    value: T,
    factors: Vec<T>
}


impl<T: Copy + Integer> Factorised<T> {
    pub fn new() -> Factorised<T> {
        return Factorised {value: T::one(), factors: Vec::new()};
    }


    pub fn create(number: T) -> Factorised<T> {
        let factors = Factorised::get_factors(number);
        Factorised {
            value: number,
            factors: factors
        }
    }


    pub fn get_factors(number: T) -> Vec<T> {
        let mut result = vec![];
        let mut remain = number;
        let mut i = T::one() + T::one();
        while (i <= remain) {
            if remain % i == T::zero() {
                remain = remain / i;
                result.push(i);
            }
            else {
                i = i + T::one();
            }
        }
        return result;
    }


    /**
     * create union of both sets
     */
    pub fn to_union(&mut self, number: T) {
        let factors = Factorised::get_factors(number);
    }

    pub fn largest_factor(&self) -> T {
        if (self.factors.is_empty()) {
            return T::one();
        }
        else {
            return self.factors[self.factors.len()];
        }
    }
}
