

use rand::Rng;

type Vector = Vec<i64>;

fn local_search(a: &Matrix, b: &Matrix, mut p: Vector, cost: &mut i64) {
    let mut i = 0;
    let mut j = 0;
    let mut nMov = 0;
    let n = p.len() as i64;
    let mut move_vec = Vec::with_capacity((n * (n - 1) / 2) as usize);
    nMov = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            move_vec.push(n * i + j);
            nMov += 1;
        }
    }
    let mut improved = true;
    for _k in 0..2 {
        if !improved {
            break;
        }
        improved = false;
        for i in 0..nMov - 1 {
            let j = rand::thread_rng().gen_range(i + 1..nMov);
            swap(&mut move_vec, i as usize, j as usize);
        }
        for i in 0..nMov {
            let r = move_vec[i as usize] / n;
            let s = move_vec[i as usize] % n;
            let d = delta(a, b, &p, r, s);
            if d < 0 {
                *cost += d;
                swap(&mut p, r as usize, s as usize);
                improved = true;
            }
        }
    }
}

fn swap(v: &mut Vec<i64>, i: usize, j: usize) {
    v.swap(i, j);
}

fn unif(low: i64, high: i64) -> i64 {
    let range = high - low + 1;
    low + (rand::thread_rng().gen::<f64>() * range as f64) as i64
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

struct Matrix {
    n: i64,
    a: Box<[i64]>,
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[i as usize * self.n as usize + j as usize]
    }
}

