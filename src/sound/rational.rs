use std::collections::HashMap;

use sound::number::*;


pub struct RationalSet<T> {
    _common_divisor: Factorised<T>,
    _numerator_set: Vec<Factorised<T>>,
}


pub struct RationalMap<K, V> {
    _common_divisor: Factorised<K>,
    _numerator_set: HashMap<Factorised<K>, V>,
}
