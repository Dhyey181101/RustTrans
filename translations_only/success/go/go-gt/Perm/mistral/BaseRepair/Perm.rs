

use rand::Rng;
use std::boxed::Box;

const fn len(p: &[i64]) -> usize {
 p.len()
}

fn perm(mut p: Vec<i64>) {
 let n = len(&p) as i64;
 p.resize(n as usize, 0);
 for i in 0..n {
 p[i as usize] = i;
 }
 let mut rng = rand::thread_rng();
 for i in 0..n {
 p.swap(i as usize, (i as i64 + rng.gen_range(0..(n-i))) as usize);
 }
}

fn swap(p: &mut [i64], i: usize, j: usize) {
 p.swap(i, j);
}

fn swap_boxed(p: &mut Box<[i64]>, i: usize, j: usize) {
 (p.as_mut()).swap(i, j);
}

fn swap_ops(p: &mut [i64], i: usize, j: usize) {
 unsafe {
 p.swap(i, j);
 }
}

