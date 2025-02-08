
use std::fmt;

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

type Vector = Vec<i64>;

fn qap_solve_sa(a: &Matrix, b: &Matrix, p: Vector, m: i64) -> i64 {
    let mut i = 0;
    let n = p.len() as i64;
    let mut w = p.clone();
    let mut cc = cost(&a, &b, &w);
    let (mut c, dmin, dmax) = init_qap(&a, &b, &mut w, &mut cc);
    let mut t0 = (dmin + (dmax - dmin) / 10) as f64;
    let tf = dmin as f64;
    let mut beta = (t0 - tf) / (m as f64 * t0 * tf);
    let mut fail = 0;
    let tries = n * (n - 1) / 2;
    let mut tfound = t0;
    let mut temp = t0;
    let mut r = 0;
    let mut s = 1;

    while i < m {
        temp /= beta * temp + 1.0;
        s += 1;
        if s >= n {
            r += 1;
            if r >= n - 1 {
                r = 0;
            }
            s = r + 1;
        }
        let d = delta(&a, &b, &w, r, s);
        if (d < 0) || (rand::random::<f64>() < (-d as f64).exp() / temp) || (fail == tries) {
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

        if c < cc {
            cc = c;
            copy(&mut w, &p);
            tfound = temp;
            if verbose() {
                println!("iteration {}: cost={}", i, cc);
                print_vector(&p);
            }
        }

        i += 1;
    }

    cc
}

fn len(v: &Vector) -> i64 {
    v.len() as i64
}

fn copy(v: &mut Vector, w: &Vector) {
    v.clone_from(w);
}

fn cost(a: &Matrix, b: &Matrix, p: &Vector) -> i64 {
    let mut c = 0;
    for i in 0..p.len() as i64 {
        for j in 0..p.len() as i64 {
            c += a.get(i, j) * b.get(p[i as usize], p[j as usize]);
        }
    }
    c
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[i as usize * self.n as usize + j as usize]
    }
}

fn init_qap(a: &Matrix, b: &Matrix, w: &mut Vector, c: &mut i64) -> (i64, i64, i64) {
    let mut dmin = 0;
    let mut dmax = 0;
    let n = w.len() as i64;
    for _ in 0..10000 {
        let r = rand::random::<i64>() % n;
        let s = rand::random::<i64>() % (n - 1);
        let d = delta(&a, &b, &w, r, s);
        *c += d;
        dmin = min(dmin, d);
        dmax = max(dmax, d);
        swap(w, r, s);
    }
    (*c, dmin, dmax)
}

fn delta(a: &Matrix, b: &Matrix, p: &Vector, r: i64, s: i64) -> i64 {
    let mut d = (a.get(r, r) - a.get(s, s)) * (b.get(p[s as usize], p[s as usize]) - b.get(p[r as usize], p[r as usize]))
        + (a.get(r, s) - a.get(s, r)) * (b.get(p[s as usize], p[r as usize]) - b.get(p[r as usize], p[s as usize]));
    for i in 0..p.len() as i64 {
        if i != r && i != s {
            d += (a.get(i, r) - a.get(i, s)) * (b.get(p[i as usize], p[s as usize]) - b.get(p[i as usize], p[r as usize]))
                + (a.get(r, i) - a.get(s, i)) * (b.get(p[s as usize], p[i as usize]) - b.get(p[r as usize], p[i as usize]));
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

fn swap(v: &mut Vector, i: i64, j: i64) {
    v.swap(i as usize, j as usize);
}

fn print_vector(v: &Vector) {
    for i in v {
        print!("{} ", i);
    }
    println!("");
}

static mut VERBOSE: bool = false;

fn verbose() -> bool {
    unsafe { VERBOSE }
}

