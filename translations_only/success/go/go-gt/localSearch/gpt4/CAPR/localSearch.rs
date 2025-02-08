
use rand::Rng;

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

type Vector = Vec<i64>;

fn local_search(a: &Matrix, b: &Matrix, p: &mut Vector, cost: &mut i64) {
    let n = p.len() as i64;
    let mut move_vec = Vec::with_capacity(((n * (n - 1)) / 2) as usize);
    let mut n_mov = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            move_vec.push(n * i + j);
            n_mov += 1;
        }
    }
    let mut improved = true;
    for _k in 0..2 {
        if !improved {
            break;
        }
        improved = false;
        for i in 0..n_mov - 1 {
            vector_swap(&mut move_vec, i as usize, unif(i + 1, n_mov - 1) as usize);
        }
        for i in 0..n_mov {
            let r = move_vec[i as usize] / n;
            let s = move_vec[i as usize] % n;
            let d = delta(a, b, p, r, s);
            if d < 0 {
                *cost += d;
                vector_swap(p, r as usize, s as usize);
                improved = true;
            }
        }
    }
}

fn unif(low: i64, high: i64) -> i64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(low..=high)
}

fn vector_swap(v: &mut Vec<i64>, i: usize, j: usize) {
    v.swap(i, j);
}

fn delta(a: &Matrix, b: &Matrix, p: &Vector, r: i64, s: i64) -> i64 {
    let mut d = (matrix_get(a, r, r) - matrix_get(a, s, s)) * (matrix_get(b, p[s as usize] as i64, p[s as usize] as i64) - matrix_get(b, p[r as usize] as i64, p[r as usize] as i64))
        + (matrix_get(a, r, s) - matrix_get(a, s, r)) * (matrix_get(b, p[s as usize] as i64, p[r as usize] as i64) - matrix_get(b, p[r as usize] as i64, p[s as usize] as i64));
    for i in 0..p.len() as i64 {
        if i != r && i != s {
            d += (matrix_get(a, i, r) - matrix_get(a, i, s)) * (matrix_get(b, p[i as usize] as i64, p[s as usize] as i64) - matrix_get(b, p[i as usize] as i64, p[r as usize] as i64))
                + (matrix_get(a, r, i) - matrix_get(a, s, i)) * (matrix_get(b, p[s as usize] as i64, p[i as usize] as i64) - matrix_get(b, p[r as usize] as i64, p[i as usize] as i64));
        }
    }
    d
}

fn matrix_get(m: &Matrix, i: i64, j: i64) -> i64 {
    m.a[(i * m.n + j) as usize]
}
