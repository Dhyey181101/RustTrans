

use rand::Rng;

type Vector = Vec<i64>;

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

fn init_qap(a: &Matrix, b: &Matrix, mut w: Vector, mut c: i64) -> (i64, i64, i64) {
    let mut dmin = i64::MAX;
    let mut dmax = i64::MIN;
    let n = w.len() as i64;
    for _ in 0..10_000 {
        let r = rand::thread_rng().gen_range(0..n);
        let mut s = rand::thread_rng().gen_range(0..(n - 1));
        if s >= r {
            s += 1;
        }
        let d = delta(a, b, &w, r, s);
        c += d;
        dmin = min(dmin, d);
        dmax = max(dmax, d);
        swap(&mut w, r as usize, s as usize);
    }
    (c, dmin, dmax)
}

fn delta(a: &Matrix, b: &Matrix, p: &[i64], r: i64, s: i64) -> i64 {
    let mut d = (get(a, r, r) - get(a, s, s)) * (get(b, p[s as usize], p[s as usize]) - get(b, p[r as usize], p[r as usize]))
        + (get(a, r, s) - get(a, s, r)) * (get(b, p[s as usize], p[r as usize]) - get(b, p[r as usize], p[s as usize]));
    for i in 0..(p.len() as i64) {
        if i != r && i != s {
            d += (get(a, i, r) - get(a, i, s)) * (get(b, p[i as usize], p[s as usize]) - get(b, p[i as usize], p[r as usize]))
                + (get(a, r, i) - get(a, s, i)) * (get(b, p[s as usize], p[i as usize]) - get(b, p[r as usize], p[i as usize]));
        }
    }
    d
}

fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m.a[i as usize * m.n as usize + j as usize]
}

fn min(a: i64, b: i64) -> i64 {
    if a < b { a } else { b }
}

fn max(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn swap(v: &mut Vec<i64>, i: usize, j: usize) {
    v.swap(i, j);
}


