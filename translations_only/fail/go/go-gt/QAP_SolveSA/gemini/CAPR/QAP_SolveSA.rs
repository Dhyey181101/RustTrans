
use rand::Rng;
use std::cmp::{max, min};
use std::fmt::Write;

fn qap_solve_sa(a: &Matrix, b: &Matrix, p: &mut Vec<i64>, m: i64) -> i64 {
    let n = p.len() as i64;
    let mut w = p.clone();
    let mut cc = cost(a, b, &p);
    let (mut c, dmin, dmax) = init_qap(a, b, &mut w, cc);
    let mut t0 = (dmin + (dmax - dmin) / 10) as f64;
    let tf = dmin as f64;
    let mut beta = (t0 - tf) / (m as f64 * t0 * tf);
    let mut fail = 0;
    let tries = n * (n - 1) / 2;
    let mut tfound = t0;
    let mut temp = t0;
    let mut r = 0;
    let mut s = 1;

    for i in 0..m {
        temp /= (beta * temp + 1.0);
        s += 1;
        if s >= n {
            r += 1;
            if r >= n - 1 {
                r = 0;
            }
            s = r + 1;
        }
        let d = delta(a, b, &mut w, r as usize, s as usize);
        if d < 0 || rand::thread_rng().gen::<f64>() < (-d as f64).exp() / temp || fail == tries {
            c += d;
            w.swap(r as usize, s as usize);
            fail = 0;
        } else {
            fail += 1;
        }
        if fail == tries {
            beta = 0.0;
            temp = tfound;
        }

        if c < cc {
            cc = c;
            *p = w.clone();
            tfound = temp;
            if Verbose {
                println!("iteration {}: cost={}", i, cc);
                print_vec(&p);
            }
        }
    }
    cc
}

fn cost(a: &Matrix, b: &Matrix, p: &[i64]) -> i64 {
    let mut c = 0;
    for i in 0..p.len() {
        for j in 0..p.len() {
            c += a.get(i as i64, j as i64) * b.get(p[i], p[j]);
        }
    }
    c
}

fn init_qap(a: &Matrix, b: &Matrix, w: &mut Vec<i64>, mut c: i64) -> (i64, i64, i64) {
    let mut dmin = i64::MAX;
    let mut dmax = i64::MIN;
    let n = w.len() as i64;
    for _ in 0..10000 {
        let r = rand::thread_rng().gen_range(0..n);
        let mut s = rand::thread_rng().gen_range(0..n - 1);
        if s >= r {
            s += 1;
        }
        let d = delta(a, b, w, r as usize, s as usize);
        c += d;
        dmin = min(dmin, d);
        dmax = max(dmax, d);
        w.swap(r as usize, s as usize);
    }
    (c, dmin, dmax)
}

fn delta(a: &Matrix, b: &Matrix, w: &mut Vec<i64>, r: usize, s: usize) -> i64 {
    let mut d = (a.get(r as i64, r as i64) - a.get(s as i64, s as i64))
        * (b.get(w[s], w[s]) - b.get(w[r], w[r]))
        + (a.get(r as i64, s as i64) - a.get(s as i64, r as i64))
            * (b.get(w[s], w[r]) - b.get(w[r], w[s]));
    for i in 0..w.len() {
        if i != r && i != s {
            d += (a.get(i as i64, r as i64) - a.get(i as i64, s as i64))
                * (b.get(w[i], w[s]) - b.get(w[i], w[r]))
                + (a.get(r as i64, i as i64) - a.get(s as i64, i as i64))
                    * (b.get(w[s], w[i]) - b.get(w[r], w[i]));
        }
    }
    d
}

fn print_vec(v: &[i64]) {
    let mut s = String::new();
    for &x in v {
        write!(&mut s, "{} ", x).unwrap();
    }
    println!("{}", s);
}

const Verbose: bool = false;

struct Vector {
    v: Vec<i64>,
}

impl Vector {
    fn len(&self) -> usize {
        self.v.len()
    }

    fn copy(&mut self, w: &Vector) {
        self.v.copy_from_slice(&w.v);
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.v.swap(i, j);
    }
}

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}
