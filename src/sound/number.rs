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

    pub fn val(&self) -> T {
        return self.value;
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
        let mut new_factors = Vec::new();

        let mut this_index = 0;
        let mut other_index = 0;
        while this_index < self.factors.len() && other_index < factors.len() {
            if (self.factors[this_index] == factors[other_index]) {
                new_factors.push(self.factors[this_index]);
                this_index += 1;
                other_index += 1;
            }
            else if (self.factors[this_index] < factors[other_index]) {
                new_factors.push(self.factors[this_index]);
                this_index += 1;
            }
            else {
                new_factors.push(factors[other_index]);
                other_index += 1;
            }
        }
        for i in this_index..self.factors.len() {
            new_factors.push(self.factors[i]);
        }
        for i in other_index..factors.len() {
            new_factors.push(factors[i]);
        }
        self.factors = new_factors;
        self.value = T::one();
        for f in &self.factors {
            self.value = self.value * T::from(*f);
        }
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
