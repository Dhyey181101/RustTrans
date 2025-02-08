

use rand::Rng;

type Vector = Vec<i64>;

fn local_search(a: &Box<Matrix>, b: &Box<Matrix>, mut p: Vector, cost: &mut i64) {
    let mut i = 0;
    let mut j = 0;
    let mut nMov = 0;
    let n = p.len() as i64;
    let mut move_vec = Vec::with_capacity((n * (n - 1) / 2).try_into().unwrap());
    nMov = 0;
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
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
        for i in 0..(nMov - 1) {
            move_vec.swap(i, rand::thread_rng().gen_range(i + 1..nMov));
        }
        for i in 0..nMov {
            let r = move_vec[i as usize] / n;
            let s = move_vec[i as usize] % n;
            let d = delta(a, b, &mut p, r, s);
            if d < 0 {
                *cost += d;
                p.swap(r as usize, s as usize);
                improved = true;
            }
        }
    }
}

fn delta(a: &Box<Matrix>, b: &Box<Matrix>, p: &mut Vector, r: i64, s: i64) -> i64 {
    let mut d = 0;
    let n = p.len() as i64;
    for i in 0..n {
        if i != r && i != s {
            d += (get(a, i, r) - get(a, i, s)) * (get(b, p[i as usize], p[s as usize]) - get(b, p[i as usize], p[r as usize]))
                + (get(a, r, i) - get(a, s, i)) * (get(b, p[s as usize], p[i as usize]) - get(b, p[r as usize], p[i as usize]));
        }
    }
    d += (get(a, r, r) - get(a, s, s)) * (get(b, p[s as usize], p[s as usize]) - get(b, p[r as usize], p[r as usize]))
        + (get(a, r, s) - get(a, s, r)) * (get(b, p[s as usize], p[r as usize]) - get(b, p[r as usize], p[s as usize]));
    d
}

struct Matrix {
    n: usize,
    a: Vec<i64>,
}

fn get(m: &Box<Matrix>, i: i64, j: i64) -> i64 {
    m.a[(i * m.n as i64 + j) as usize]
}


