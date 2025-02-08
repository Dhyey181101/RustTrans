

use rand::Rng;

const VERBOSE: bool = false;

fn qap_solve_sa(a: Box<Matrix>, b: Box<Matrix>, mut p: Vec<i64>, m: i64) -> i64 {
    let n = p.len() as i64;
    let mut w = p.clone();
    let mut cc = cost(&a, &b, &p);
    let (mut c, dmin, dmax) = init_qap(&a, &b, &mut w, cc);
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
        if d < 0 || rand::thread_rng().gen_bool((-d as f64 / temp).exp()) || fail == tries {
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
            p = w.clone();
            tfound = temp;
            if VERBOSE {
                println!("iteration {}: cost={}", i, cc);
                print_vector(&p);
            }
        }
    }
    cc
}

fn cost(a: &Matrix, b: &Matrix, p: &[i64]) -> i64 {
    let mut c = 0;
    for i in 0..p.len() as i64 {
        for j in 0..p.len() as i64 {
            c += a.get(i, j) * b.get(p[i as usize], p[j as usize]);
        }
    }
    c
}

fn init_qap(a: &Matrix, b: &Matrix, w: &mut Vec<i64>, c: i64) -> (i64, i64, i64) {
    let mut dmin = i64::MAX;
    let mut dmax = i64::MIN;
    let n = w.len() as i64;
    for _ in 0..10000 {
        let r = rand::thread_rng().gen_range(0..n);
        let mut s = rand::thread_rng().gen_range(0..n - 1);
        if s >= r {
            s += 1;
        }
        let d = delta(a, b, w, r, s);
        let _c = c + d;
        dmin = dmin.min(d);
        dmax = dmax.max(d);
        w.swap(r as usize, s as usize);
    }
    (c, dmin, dmax)
}

fn delta(a: &Matrix, b: &Matrix, p: &[i64], r: i64, s: i64) -> i64 {
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

fn print_vector(v: &[i64]) {
    for &x in v {
        print!("{} ", x);
    }
    println!();
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


