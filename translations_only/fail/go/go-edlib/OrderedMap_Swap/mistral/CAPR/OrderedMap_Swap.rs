

use std::boxed::Box;
use std::mem;

#[derive(Clone)]
struct Pair {
 key: String,
 value: f32,
}

type OrderedMap = Vec<Box<Pair>>;

pub fn swap_map_elements(map: &mut OrderedMap, i: usize, j: usize) {
 let temp = map[i].clone();
 map[i] = mem::replace(&mut map[j], Box::new(Pair {
 key: temp.key,
 value: temp.value,
 }));
}

