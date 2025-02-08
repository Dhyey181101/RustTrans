
use std::vec::Vec;

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

fn get(g: &Matrix, i: i64, j: i64) -> i64 {
    g.a[i as usize * g.n as usize + j as usize]
}

fn set(g: &mut Matrix, i: i64, j: i64, v: i64) {
    g.a[i as usize * g.n as usize + j as usize] = v;  
}

fn transitive_closure(g: &mut Matrix, n: &mut Option<Box<Matrix>>) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < g.n {
        while j < g.n {
            while k < g.n {
                let gi_k = get(g, i, k);
                if gi_k > 0 {
                    let gk_j = get(g, k, j);
                    if gk_j > 0 {
                        let gi_j = get(g, i, j);
                        if gi_j == 0 || gi_k + gk_j < gi_j {
                            set(g, i, j, gi_k + gk_j);
                            if let Some(ref mut m) = *n {
                                set(m, i, j, k + 1);
                            }
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

