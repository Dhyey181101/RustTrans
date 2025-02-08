

use rand::Rng;
use std::boxed::Box;

const fn len(p: &[i64]) -> usize {
 p.len()
}

fn perm(mut p: Vec<i64>) {
 let mut rng = rand::thread_rng();
 let n = len(&p) as i64;
 for i in 0..n {
 let j = rng.gen_range(0..n) as usize;
 p.swap(i as usize, j);
 }
}

