

use rand::Rng;
use std::fmt;

const EPSILON: f64 = 1e-8;

fn gen_trace(p: &mut Vec<usize>, trace: &Vec<Vec<i64>>) {
    let n = p.len();
    let mut nexti = vec![0; n];
    let mut nextj = vec![0; n];
    let mut sum_trace = vec![0; n];

    nexti.copy_from_slice(&random_permutation(&(0..n).collect::<Vec<usize>>()));
    nextj.copy_from_slice(&random_permutation(&(0..n).collect::<Vec<usize>>()));
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

fn random_permutation(v: &Vec<usize>) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let n = v.len();
    let mut p = v.clone();
    for i in 0..n {
        p[i] = i;
    }
    for i in 0..n {
        p.swap(i, i + rng.gen_range(0..n-i));
    }
    p
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

fn unif(low: i64, high: i64) -> i64 {
    low + (high - low + 1) as i64 * rand::thread_rng().gen::<f64>() as i64
}

#[derive(Debug)]
struct Vector(Vec<i64>);

impl Vector {
    fn new(n: usize) -> Self {
        Vector(vec![0; n])
    }

    fn as_slice(&self) -> &[i64] {
        &self.0
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.0.swap(i, j);
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.0.len() {
            write!(f, "{} ", self.0[i])?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Matrix(Vec<Vec<i64>>);

impl Matrix {
    fn new(n: usize) -> Self {
        Matrix(vec![vec![0; n]; n])
    }

    fn as_slice(&self) -> &[Vec<i64>] {
        &self.0
    }

    fn get(&self, i: usize, j: usize) -> i64 {
        self.0[i][j]
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.0.len() {
            for j in 0..self.0[i].len() {
                write!(f, "{} ", self.0[i][j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

