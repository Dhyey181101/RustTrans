

use rand::Rng;
use std::boxed::Box;

const fn len(p: &[i64]) -> usize {
    p.len()
}

fn perm(mut p: Vec<i64>) {
    let n = len(&p) as i64;
    p.resize(n as usize, 0);
    let mut rng = rand::thread_rng();
    for i in 0..n {
        p[i as usize] = i;
    }
    for i in 0..n {
        let j: usize = (i as i64 + rng.gen_range(0..n-i)) as usize;
        p.swap(i as usize, j);
    }
}

fn swap(p: &mut [i64], i: usize, j: usize) {
    let temp = p[i];
    p[i] = p[j];
    p[j] = temp;
}

fn swap_boxed(p: &mut Box<[i64]>, i: usize, j: usize) {
    let temp = (*p)[i];
    (*p)[i] = (*p)[j];
    (*p)[j] = temp;
}

fn swap_using_swap_ops(p: &mut [i64], i: usize, j: usize) {
    unsafe {
        p.swap(i, j);
    }
}

