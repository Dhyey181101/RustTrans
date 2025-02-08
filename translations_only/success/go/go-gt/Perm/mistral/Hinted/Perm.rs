

use rand::Rng;
use std::boxed::Box;

const fn len(p: &[i64]) -> usize {
 p.len()
}

fn perm(mut p: Vec<i64>) {
 let n = len(&p) as i64;
 let mut p = Box::new(p);
 for i in 0..n {
 p[i as usize] = i;
 }
 let mut rng = rand::thread_rng();
 for i in 0..n {
 let j: usize = i as usize + rng.gen_range(0..(n - i)) as usize;
 p.swap(i as usize, j); // use swap method of Vec
 }
}

fn swap(p: &mut Vec<i64>, i: usize, j: usize) {
 let x = p[i].clone();
 p[i] = p[j].clone();
 p[j] = x;
}

