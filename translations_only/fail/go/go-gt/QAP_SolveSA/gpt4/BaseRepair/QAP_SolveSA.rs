
use rand::Rng;
use std::f64;

static mut VERBOSE: bool = false;

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

struct Vector(Vec<i64>);

impl Vector {
    fn len(&self) -> i64 {
        self.0.len() as i64
    }

    fn copy(&mut self, other: &Vector) {
        self.0.clone_from(&other.0);
    }

    fn swap(&mut self, i: i64, j: i64) {
        self.0.swap(i as usize, j as usize);
    }

    fn print(&self) {
        for &item in &self.0 {
            print!("{} ", item);
        }
        println!();
    }
}

fn qap_solve_sa(a: &Matrix, b: &Matrix, p: &mut Vector, m: i64) -> i64 {
    let mut rng = rand::thread_rng();
    let n = p.len();
    let mut w = Vector(p.0.clone());
    let mut cc = cost(a, b, p);
    let (mut c, dmin, dmax) = init_qap(a, b, &mut w, cc);
    let t0 = dmin as f64 + (dmax - dmin) as f64 / 10.0;
    let tf = dmin as f64;
    let beta = (t0 - tf) / (m as f64 * t0 * tf);
    let mut fail = 0;
    let tries = n * (n - 1) / 2;
    let mut tfound = t0;
    let mut temp = t0;
    let mut r = 0;
    let mut s = 1;

    for i in 0..m {
        temp /= beta * temp + 1.0;
        s += 1;
        if s >= n {
            r += 1;
            if r >= n - 1 {
                r = 0;
            }
            s = r + 1;
        }
        let d = delta(a, b, &w, r, s);
        if d < 0 || rng.gen::<f64>() < f64::exp(-(d as f64) / temp) || fail == tries {
            c += d;
            w.swap(r, s);
            fail = 0;
        } else {
            fail += 1;
        }
        if fail == tries {
            temp = tfound;
        }

        if c < cc {
            cc = c;
            p.copy(&w);
            tfound = temp;
            unsafe {
                if VERBOSE {
                    println!("iteration {}: cost={}", i, cc);
                    p.print();
                }
            }
        }
    }
    cc
}

fn cost(a: &Matrix, b: &Matrix, p: &Vector) -> i64 {
    let mut c = 0;
    for i in 0..p.len() {
        for j in 0..p.len() {
            c += a.get(i, j) * b.get(p.0[i as usize], p.0[j as usize]);
        }
    }
    c
}

fn init_qap(a: &Matrix, b: &Matrix, w: &mut Vector, mut c: i64) -> (i64, i64, i64) {
    let mut rng = rand::thread_rng();
    let n = w.len();
    let mut dmin = i64::MAX;
    let mut dmax = i64::MIN;
    for _ in 0..10000 {
        let r = rng.gen_range(0..n);
        let mut s = rng.gen_range(0..n - 1);
        if s >= r {
            s += 1;
        }
        let d = delta(a, b, w, r, s);
        c += d;
        dmin = std::cmp::min(dmin, d);
        dmax = std::cmp::max(dmax, d);
        w.swap(r, s);
    }
    (c, dmin, dmax)
}

fn delta(a: &Matrix, b: &Matrix, p: &Vector, r: i64, s: i64) -> i64 {
    let mut d = (a.get(r, r) - a.get(s, s)) * (b.get(p.0[s as usize], p.0[s as usize]) - b.get(p.0[r as usize], p.0[r as usize]))
        + (a.get(r, s) - a.get(s, r)) * (b.get(p.0[s as usize], p.0[r as usize]) - b.get(p.0[r as usize], p.0[s as usize]));
    for i in 0..p.len() {
        if i != r && i != s {
            d += (a.get(i, r) - a.get(i, s)) * (b.get(p.0[i as usize], p.0[s as usize]) - b.get(p.0[i as usize], p.0[r as usize]))
                + (a.get(r, i) - a.get(s, i)) * (b.get(p.0[s as usize], p.0[i as usize]) - b.get(p.0[r as usize], p.0[i as usize]));
        }
    }
    d
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}
