
use rand::Rng;

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
        let d = delta(a, b, w, r, s);
        c += d;
        dmin = dmin.min(d);
        dmax = dmax.max(d);
        w.swap(r as usize, s as usize);
    }
    (c, dmin, dmax)
}

fn delta(a: &Matrix, b: &Matrix, p: &Vec<i64>, r: i64, s: i64) -> i64 {
    let mut d = 0;
    d += (a.get(r, r) - a.get(s, s)) * (b.get(p[s as usize], p[s as usize]) - b.get(p[r as usize], p[r as usize]));
    d += (a.get(r, s) - a.get(s, r)) * (b.get(p[s as usize], p[r as usize]) - b.get(p[r as usize], p[s as usize]));
    for i in 0..p.len() {
        if i != r as usize && i != s as usize {
            d += (a.get(i as i64, r) - a.get(i as i64, s)) * (b.get(p[i], p[s as usize]) - b.get(p[i], p[r as usize]));
            d += (a.get(r, i as i64) - a.get(s, i as i64)) * (b.get(p[s as usize], p[i]) - b.get(p[r as usize], p[i]));
        }
    }
    d
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
