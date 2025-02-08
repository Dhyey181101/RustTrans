

use rand::Rng;

type Vector = Vec<i64>;

fn init_qap(a: Box<Matrix>, b: Box<Matrix>, mut w: Vector, c: i64) -> (i64, i64, i64) {
    let mut dmin: i64 = i64::MAX;
    let mut dmax: i64 = i64::MIN;
    let n = w.len() as i64;
    let mut rng = rand::thread_rng();
    for _ in 0..10000 {
        let r = rng.gen_range(0..n);
        let mut s = rng.gen_range(0..(n - 1));
        if s >= r {
            s += 1;
        }
        let d = delta(&*a, &*b, &w, r, s);
        let c = c + d;
        dmin = min(dmin, d);
        dmax = max(dmax, d);
        swap_vector_elements(&mut w, r as usize, s as usize);
    }
    (c, dmin, dmax)
}

fn delta(a: &Matrix, b: &Matrix, p: &Vector, r: i64, s: i64) -> i64 {
    let mut d = (a.a[(r * a.n + r) as usize] - a.a[(s * a.n + s) as usize]) * (b.a[(p[s as usize] * b.n + p[s as usize]) as usize] - b.a[(p[r as usize] * b.n + p[r as usize]) as usize])
        + (a.a[(r * a.n + s) as usize] - a.a[(s * a.n + r) as usize]) * (b.a[(p[s as usize] * b.n + p[r as usize]) as usize] - b.a[(p[r as usize] * b.n + p[s as usize]) as usize]);
    for i in 0..p.len() as i64 {
        if i != r && i != s {
            d += (a.a[(i * a.n + r) as usize] - a.a[(i * a.n + s) as usize]) * (b.a[(p[i as usize] * b.n + p[s as usize]) as usize] - b.a[(p[i as usize] * b.n + p[r as usize]) as usize])
                + (a.a[(r * a.n + i) as usize] - a.a[(s * a.n + i) as usize]) * (b.a[(p[s as usize] * b.n + p[i as usize]) as usize] - b.a[(p[r as usize] * b.n + p[i as usize]) as usize]);
        }
    }
    d
}

fn min(a: i64, b: i64) -> i64 {
    if a < b {
        a
    } else {
        b
    }
}

fn max(a: i64, b: i64) -> i64 {
    if a > b {
        a
    } else {
        b
    }
}

fn swap_vector_elements(v: &mut Vector, i: usize, j: usize) {
    let x = v[i];
    v[i] = v[j];
    v[j] = x;
}

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

