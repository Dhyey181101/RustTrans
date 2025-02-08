

use rand::prelude::*;
use std::boxed::Box;

const NOT_VISITED: i64 = -1;

type Vector = Vec<i64>;
type Matrix = Box<[i64]>;

static mut RNG: Option<ThreadRng> = None;

fn local_search(a: &Matrix, b: &Matrix, p: &mut Vector) {
    let n = p.len() as i64;
    let mut moves = generate_moves(n, p);
    let mut improved = true;
    for _ in 0..2 {
        if !improved {
            break;
        }
        improve(&mut moves, n, a, b, p);
        improved = false;
    }
}

fn generate_moves(n: i64, p: &Vector) -> Vector {
    let mut moves = Vec::new();
    for i in 0..n - 1 {
        for j in (i + 1)..n {
            if p[i as usize] != p[j as usize] {
                moves.push(n * i + j);
            }
        }
    }
    moves
}

fn improve(
    moves: &mut Vector,
    n: i64,
    a: &Matrix,
    b: &Matrix,
    p: &mut Vector,
) {
    let mut rng = get_rng();
    moves.shuffle(&mut rng);
    for i in 0..moves.len() {
        let r = moves[i] / n;
        let s = moves[i] % n;
        let d = delta(a, b, p, r, s);
        if d < 0 {
            p[r as usize] = p[s as usize];
        }
    }
}

fn len(v: &Vector) -> i64 {
    v.len() as i64
}

fn unif(low: i64, high: i64) -> i64 {
    low + rand::thread_rng().gen_range(0..=high-low)
}

fn swap(p: &mut Vector, i: usize, j: usize) {
    let temp = p[i];
    p[i] = p[j];
    p[j] = temp;
}

fn delta(
    a: &Matrix,
    b: &Matrix,
    p: &Vector,
    r: i64,
    s: i64,
) -> i64 {
    let mut d = 0;
    let n = len(p);
    for i in 0..n {
        if i != r && i != s {
            d += (get(a, i, r) - get(a, i, s)) *
                (get(b, p[i as usize], 0) - get(b, p[s as usize], 0)) +
                (get(a, r, i) - get(a, s, i)) *
                (get(b, p[s as usize], 0) - get(b, p[r as usize], 0));
        }
    }
    (get(a, r, r) - get(a, s, s)) *
        (get(b, p[r as usize], 0) - get(b, p[s as usize], 0)) +
        (get(a, r, s) - get(a, s, r)) *
        (get(b, p[s as usize], 0) - get(b, p[r as usize], 0)) +
        d
}

fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m[i as usize * m.len()/8 + j as usize]
}

fn get_rng() -> &'static mut ThreadRng {
    unsafe {
        RNG.get_or_insert_with(|| thread_rng());
        RNG.as_mut().unwrap()
    }
}

