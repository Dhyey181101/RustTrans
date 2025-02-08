

use rand::prelude::*;
use std::fmt;

const EPSILON: f64 = 1e-8;

fn gen_trace(p: &mut Vec<usize>, trace: &Vec<Vec<i64>>) {
    let n = p.len();
    let mut nexti = vec![0; n];
    let mut nextj = vec![0; n];
    let mut sum_trace = vec![0; n];

    nexti.copy_from_slice(&random_permutation(n));
    nextj.copy_from_slice(&random_permutation(n));
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

fn random_permutation(n: usize) -> Vec<usize> {
    let mut p = (0..n).collect::<Vec<usize>>();
    let mut rng = thread_rng();
    for i in 0..n {
        p.swap(i, i + rng.gen_range(0..n - i));
    }
    p
}

struct Vector([i64; 128]);

impl Vector {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.len() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", self.0[i])?;
        }
        Ok(())
    }
}

struct Matrix {
    n: usize,
    a: Vec<i64>,
}

impl Matrix {
    fn get(&self, i: usize, j: usize) -> i64 {
        self.a[i * self.n + j]
    }
}

fn unif(low: i64, high: i64) -> i64 {
    low + (high - low + 1) as i64 * rand::thread_rng().gen::<f64>() as i64
}

