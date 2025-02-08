
use std::ops::IndexMut;

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
        j = 0;
        while j < g.n {
            k = 0;
            while k < g.n {
                if get(g, i, k) > 0 && get(g, k, j) > 0 {
                    if get(g, i, j) == 0 || get(g, i, k) + get(g, k, j) < get(g, i, j) {
                        set(g, i, j, get(g, i, k) + get(g, k, j));
                        if let Some(ref mut m) = n {
                            set(m, i, j, k + 1);
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

