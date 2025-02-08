
use rand::distributions::Uniform;
use rand::prelude::*;

fn local_search(a: &Matrix, b: &Matrix, p: &mut Vec<i64>, cost: &mut i64) {
    let mut n_mov = 0;
    let n = p.len();
    let mut move_vec = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n - 1 {
        for j in i + 1..n {
            move_vec.push(n * i + j);
            n_mov += 1;
        }
    }
    let mut improved = true;
    for k in 0..2 {
        if !improved {
            break;
        }
        improved = false;
        for i in 0..n_mov - 1 {
            move_vec.swap(i, thread_rng().sample(Uniform::new(i + 1, n_mov)));
        }
        for i in 0..n_mov {
            let r = move_vec[i] / n;
            let s = move_vec[i] % n;
            let d = delta(a, b, p, r as i64, s as i64);
            if d < 0 {
                *cost += d;
                p.swap(r as usize, s as usize);
                improved = true;
            }
        }
    }
}

fn unif(low: i64, high: i64) -> i64 {
    low + thread_rng().gen_range(low..=high)
}

fn delta(a: &Matrix, b: &Matrix, p: &Vec<i64>, r: i64, s: i64) -> i64 {
    let mut d = (a.get(r, r) - a.get(s, s)) * (b.get(p[s as usize], p[s as usize]) - b.get(p[r as usize], p[r as usize]))
        + (a.get(r, s) - a.get(s, r)) * (b.get(p[s as usize], p[r as usize]) - b.get(p[r as usize], p[s as usize]));
    for i in 0..p.len() {
        if i != r as usize && i != s as usize {
            d += (a.get(i as i64, r) - a.get(i as i64, s)) * (b.get(p[i], p[s as usize]) - b.get(p[i], p[r as usize]))
                + (a.get(r, i as i64) - a.get(s, i as i64)) * (b.get(p[s as usize], p[i]) - b.get(p[r as usize], p[i]));
        }
    }
    d
}

#[derive(Debug)]
struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[i as usize * self.n as usize + j as usize]
    }
}
