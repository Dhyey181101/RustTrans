
use rand::Rng;

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}

type Vector = Vec<i64>;

fn init_qap(a: &Matrix, b: &Matrix, w: &mut Vector, mut c: i64) -> (i64, i64, i64) {
    let mut dmin = i64::MAX;
    let mut dmax = i64::MIN;
    let n = w.len() as i64;
    let mut rng = rand::thread_rng();
    for _ in 0..10000 {
        let r = rng.gen_range(0..n);
        let mut s = rng.gen_range(0..n - 1);
        if s >= r {
            s += 1;
        }
        let d = delta(a, b, w, r, s);
        c += d;
        dmin = min(dmin, d);
        dmax = max(dmax, d);
        swap(w, r, s);
    }
    (c, dmin, dmax)
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

fn swap(p: &mut Vector, i: i64, j: i64) {
    p.swap(i as usize, j as usize);
}
