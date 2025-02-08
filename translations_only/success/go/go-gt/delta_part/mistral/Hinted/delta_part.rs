

use std::boxed::Box;
use std::ops::Index;

type Vector = Vec<i64>;

struct RustMatrix {
    n: i64,
    a: Vec<i64>,
}

impl Index<usize> for RustMatrix {
    type Output = i64;

    fn index(&self, index: usize) -> &i64 {
        &self.a[index]
    }
}

impl RustMatrix {
    fn new(n: i64) -> RustMatrix {
        RustMatrix {
            n,
            a: vec![0; (n * n as i64) as usize],
        }
    }

    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[i as usize * self.n as usize + j as usize]
    }
}

fn delta_part(
    a: &RustMatrix,
    b: &RustMatrix,
    dist: &RustMatrix,
    p: &Vector,
    i: i64,
    j: i64,
    r: i64,
    s: i64,
) -> i64 {
    let a_ri = a.a[(i * a.n as i64 + r) as usize];
    let a_rj = a.a[(i * a.n as i64 + j) as usize];
    let a_si = a.a[(s * a.n as i64 + i) as usize];
    let a_sj = a.a[(s * a.n as i64 + j) as usize];
    let b_psj = b.a[p[s as usize] as usize];
    let b_psi = b.a[p[s as usize] as usize];
    let b_pji = b.a[p[j as usize] as usize];
    let b_pri = b.a[p[r as usize] as usize];
    let b_pij = b.a[p[i as usize] as usize];
    let b_pjs = b.a[p[j as usize] as usize];
    let b_pss = b.a[p[s as usize] as usize];
    let b_pjs_b_pjs = 2 * b_pjs;
    let b_pss_b_pij = b_pss + b_pij;
    let b_pri_b_pjs = b_pri + b_pjs;
    let b_pss_b_pri = b_pss + b_pri;
    let b_pij_minus_b_pri = b_pij as i64 - b_pri as i64;
    let dist_ij = dist.a[(i * dist.n as i64 + j) as usize];
    let a_r_minus_a_j = a_ri - a_rj;
    let a_s_minus_a_i = a_si - a_sj;
    let a_i_minus_a_j = a.get(i, j) - a_rj;
    let a_j_minus_a_s = a.get(j, s) - a_si;
    let b_pjs_minus_b_pij = b_pjs as i64 - b_pij as i64;
    let b_pri_minus_b_pjs = b_pri as i64 - b_pjs as i64;
    let b_pss_minus_b_pri = b_pss as i64 - b_pri as i64;
    a_r_minus_a_j * (b_pjs_minus_b_pij + b_pri_minus_b_pjs) +
    a_s_minus_a_i * (b_pss_b_pij + b_pri_minus_b_pjs) +
    a_i_minus_a_j * (b_pss_b_pri + b_pij_minus_b_pri) +
    a_j_minus_a_s * (b_pjs_b_pjs) +
    dist_ij
}

fn main() {
    // Add test cases here
}

