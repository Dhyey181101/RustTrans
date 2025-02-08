
use std::collections::VecDeque;

pub fn swap(p: &mut VecDeque<(String, f32)>, i: isize, j: isize) {
    p.swap(i as usize, j as usize);
}

pub struct OrderedMap(pub VecDeque<(String, f32)>);
