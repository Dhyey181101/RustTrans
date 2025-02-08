
use std::collections::HashMap;
use std::iter;
use std::str;
use std::ops::Mul;

fn main() {
 let mut map = HashMap::new();
 map.insert(1, 2);
 map.insert(3, 4);

 let v: Vec<_> = map.into_iter().collect();
 let i = v[0].0 * v[0].1;

 assert_eq!(i, 2);
}
