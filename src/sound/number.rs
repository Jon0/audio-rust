use num_integer::Integer;


pub struct Factorised<T> {
    value: T,
    factors: Vec<T>
}


impl<T: Copy + Integer> Factorised<T> {
    pub fn create(number: T) -> Factorised<T> {
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
        Factorised {
            value: number,
            factors: result
        }
    }
}
