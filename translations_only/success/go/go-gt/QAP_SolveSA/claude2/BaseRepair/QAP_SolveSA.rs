
use rand::Rng;
use std::fmt;

struct Vector(Vec<i64>);

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

fn qap_solve_sa(a: &Matrix, b: &Matrix, mut p: Vector, m: i64, mut c: i64) -> i64 {
    let mut i = 0;
    let n = p.0.len() as i64;
    let mut w = Vector(vec![0; n as usize]);
    copy_vector(&mut w, &p);
    c = cost(a, b, &p);
    let (mut c, dmin, dmax) = init_qap(a, b, &mut w, c);
    let mut t0 = ((dmin + (dmax - dmin) / 10) as f64).floor();
    let mut beta = (t0 - dmin as f64) / ((m as f64) * t0 * dmin as f64);
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
                r = 0
            }
            s = r + 1;
        }
        let d = delta(a, b, &mut w, r, s);
        if (d < 0) || (rand::thread_rng().gen_bool( ((-d as f64) / temp).exp())) || (fail == tries) {
            c += d;
            swap_vector(&mut w, r, s);
            fail = 0;
        } else {
            fail += 1;
        }
        if fail == tries {
            beta = 0.0;
            temp = tfound;
        }

        if c < c {
            c = c;
            copy_vector(&mut p, &w);
            tfound = temp;
        }
    }

    c
}

fn len_vector(v: &Vector) -> i64 {
    v.0.len() as i64
}

fn copy_vector(dst: &mut Vector, src: &Vector) {
    dst.0.copy_from_slice(&src.0);
}

fn swap_vector(v: &mut Vector, i: i64, j: i64) {
    v.0.swap(i as usize, j as usize);
}

fn cost(a: &Matrix, b: &Matrix, p: &Vector) -> i64 {
    let mut c = 0;
    for i in 0..len_vector(p) {
        for j in 0..len_vector(p) {
            c += a.get(i, j) * b.get(p.0[i as usize] as i64, p.0[j as usize] as i64);
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
    let n = len_vector(w);
    for _ in 0..10000 {
        let r = rand::thread_rng().gen_range(0..n);
        let mut s = rand::thread_rng().gen_range(0..n-1);
        if s >= r {
            s += 1;
        }
        let d = delta(a, b, w, r, s);
        c += d;
        dmin = min(dmin, d);
        dmax = max(dmax, d);
        swap_vector(w, r, s);
    }
    (c, dmin, dmax)
}

fn delta(a: &Matrix, b: &Matrix, p: &mut Vector, r: i64, s: i64) -> i64 {
    let mut d = ((a.get(r, r) - a.get(s, s)) * (b.get(p.0[s as usize] as i64, p.0[s as usize] as i64) - b.get(p.0[r as usize] as i64, p.0[r as usize] as i64)))
        + ((a.get(r, s) - a.get(s, r)) * (b.get(p.0[s as usize] as i64, p.0[r as usize] as i64) - b.get(p.0[r as usize] as i64, p.0[s as usize] as i64)));
    for i in 0..len_vector(p) {
        if i != r && i != s {
            d += (a.get(i, r) - a.get(i, s)) * (b.get(p.0[i as usize] as i64, p.0[s as usize] as i64) - b.get(p.0[i as usize] as i64, p.0[r as usize] as i64))
                + (a.get(r, i) - a.get(s, i)) * (b.get(p.0[s as usize] as i64, p.0[i as usize] as i64) - b.get(p.0[r as usize] as i64, p.0[i as usize] as i64));
        }
    }
    d
}

fn min(a: i64, b: i64) -> i64 {
    if a < b { a } else { b }
}

fn max(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

