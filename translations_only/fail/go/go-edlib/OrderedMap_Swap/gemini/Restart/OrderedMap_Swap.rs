
use std::collections::VecDeque;

pub fn swap<T>(v: &mut VecDeque<T>, i: usize, j: usize) {
    v.swap(i, j);
}

#[derive(Debug, Clone)]
pub struct Pair<K, V> {
    pub key: K,
    pub value: V,
}

#[derive(Debug, Clone)]
pub struct OrderedMap<K, V> {
    pub pairs: VecDeque<Pair<K, V>>,
}
