

use rand::Rng;
use std::fmt;

const EPSILON: f64 = 1e-8;

fn gen_trace(p: &mut Vec<usize>, trace: &Vec<Vec<i64>>) {
    let n = p.len();
    let mut nexti = vec![0; n];
    let mut nextj = vec![0; n];
    let mut sum_trace = vec![0; n];

    nexti.copy_from_slice(&random_permutation(&vec![0; n]));
    nextj.copy_from_slice(&random_permutation(&vec![0; n]));
    for i in 0..n {
        for j in 0..n {
            sum_trace[i] += trace[i][j];
        }
    }

    for i in 0..n {
        let target = unif(0, sum_trace[nexti[i]] as i64 - 1);
        let mut j = i;
        let mut sum = trace[nexti[i]][nextj[j]];
        while sum < target as i64 {
            j += 1;
            sum += trace[nexti[i]][nextj[j]];
        }
        p[nexti[i]] = nextj[j];
        for k in i..n {
            sum_trace[nexti[k]] -= trace[nexti[k]][nextj[j]];
        }
        nextj.swap(j, i);
    }
}

fn random_permutation(v: &[usize]) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut v = v.to_vec();
    let len = v.len();
    for i in 0..len {
        let j = rng.gen_range(i..len);
        v.swap(i, j);
    }
    v
}

fn len(v: &Vec<usize>) -> usize {
    v.len()
}

fn swap(p: &mut Vec<usize>, i: usize, j: usize) {
    let temp = p[i];
    p[i] = p[j];
    p[j] = temp;
}

fn get(m: &Vec<Vec<i64>>, i: usize, j: usize) -> i64 {
    m[i][j]
}

fn unif(min: i64, max: i64) -> i64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

