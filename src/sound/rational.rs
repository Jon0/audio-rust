use std::collections::HashMap;

use sound::number::*;


pub struct RationalSet<T> {
    common_divisor: Factorised<T>,
    numerator_set: Vec<Factorised<T>>,
}


pub struct RationalMap<K, V> {
    common_divisor: Factorised<K>,
    numerator_set: HashMap<Factorised<K>, V>,
}
