

use rand::Rng;
use std::fmt;

struct Vector(Vec<i64>);

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

fn qap_solve_sa(a: &Matrix, b: &Matrix, mut p: Vector, m: i64) -> i64 {
    let mut i = 0;
    let n = p.0.len() as i64;
    let mut w = Vector(vec![0; n as usize]);
    copy(&mut w, &p);
    let mut cc = cost(a, b, &p);
    let (mut c, dmin, dmax) = init_qap(a, b, &mut w, cc);
    let mut t0 = ((dmax - dmin) as f64 / 10.0 + dmin as f64) as f64;
    let tf = dmin as f64;
    let mut beta = (t0 - tf) / (m as f64 * t0 * tf);
    let mut fail = 0;
    let tries = n * (n - 1) / 2;
    let mut tfound = t0;
    let mut temp = t0;
    let mut r = 0;
    let mut s = 1;

    // SA iterations
    while i < m {
        temp /= beta * temp + 1.0;
        s += 1;
        if s >= n {
            r += 1;
            if r >= n - 1 {
                r = 0
            }
            s = r + 1;
        }
        let d = delta(a, b, &w, r, s);
        if d < 0 || rand::thread_rng().gen_bool( ((-d) as f64 / temp).exp()) || fail == tries {
            c += d;
            swap(&mut w, r, s);
            fail = 0;
        } else {
            fail += 1;
        }
        if fail == tries {
            beta = 0.0;
            temp = tfound;
        }

        // Best solution improved ?
        if c < cc {
            cc = c;
            copy(&mut p, &w);
            tfound = temp;
            if false {
                print_vector(&p);
            }
        }
        i += 1;
    }
    cc
}

fn copy(dst: &mut Vector, src: &Vector) {
    dst.0.copy_from_slice(&src.0);
}

fn swap(v: &mut Vector, i: i64, j: i64) {
    v.0.swap(i as usize, j as usize);
}

fn len(v: &Vector) -> i64 {
    v.0.len() as i64
}

fn print_vector(v: &Vector) {
    for i in &v.0 {
        print!("{} ", i);
    }
    println!("");
}

fn cost(a: &Matrix, b: &Matrix, p: &Vector) -> i64 {
    let mut c = 0;
    for i in 0..len(p) {
        for j in 0..len(p) {
            c += a.get(i, j) * b.get(p.0[i as usize], p.0[j as usize]);
        }
    }
    c
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}

fn init_qap(a: &Matrix, b: &Matrix, w: &mut Vector, mut c: i64) -> (i64, i64, i64) {
    let mut dmin = 0;
    let mut dmax = 0;
    let n = len(w);
    for _ in 0..10000 {
        let r = rand::thread_rng().gen_range(0..n);
        let mut s = rand::thread_rng().gen_range(0..n-1);
        if s >= r {
            s += 1;
        }
        let d = delta(a, b, w, r, s);
        c += d;
        dmin = i64_min(dmin, d);
        dmax = i64_max(dmax, d);
        swap(w, r, s);
    }
    (c, dmin, dmax)
}

fn delta(a: &Matrix, b: &Matrix, p: &Vector, r: i64, s: i64) -> i64 {
    let mut d = (a.get(r, r) - a.get(s, s)) * (b.get(p.0[s as usize], p.0[s as usize]) - b.get(p.0[r as usize], p.0[r as usize]))
        + (a.get(r, s) - a.get(s, r)) * (b.get(p.0[s as usize], p.0[r as usize]) - b.get(p.0[r as usize], p.0[s as usize]));
    for i in 0..len(p) {
        if i != r && i != s {
            d += (a.get(i, r) - a.get(i, s)) * (b.get(p.0[i as usize], p.0[s as usize]) - b.get(p.0[i as usize], p.0[r as usize]))
                + (a.get(r, i) - a.get(s, i)) * (b.get(p.0[s as usize], p.0[i as usize]) - b.get(p.0[r as usize], p.0[i as usize]));
        }
    }
    d
}

fn i64_min(a: i64, b: i64) -> i64 {
    if a < b { a } else { b }
}

fn i64_max(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

