
use std::boxed::Box;

struct Matrix {
    n: i64,
    a: Box<[i64]>,
}

fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m.a[(i * m.n + j) as usize]
}

fn set(m: &mut Matrix, i: i64, j: i64, v: i64) {
    m.a[(i * m.n + j) as usize] = v;
}

fn transitive_closure(g: &mut Matrix, mut n: Option<Box<Matrix>>) {
    let mut i: i64 = 0;
    while i < g.n {
        let mut j: i64 = 0;
        while j < g.n {
            let mut k: i64 = 0;
            while k < g.n {
                if get(g, i, k) > 0 && get(g, k, j) > 0 {
                    if get(g, i, j) == 0 || get(g, i, k) + get(g, k, j) < get(g, i, j) {
                        set(g, i, j, get(g, i, k) + get(g, k, j));
                        if let Some(ref mut n) = n {
                            set(n, i, j, k + 1);
                        }
                    }
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
}
