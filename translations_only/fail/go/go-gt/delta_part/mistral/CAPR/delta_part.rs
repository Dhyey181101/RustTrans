

use std::mem;

type Vector = Vec<i64>;
type Matrix = Vec<i64>;

const NOT_IMPL: i64 = -1;

fn delta_part(
    a: &Matrix, b: &Matrix, dist: &Matrix, p: &Vector, i: i64, j: i64, r: i64, s: i64
) -> i64 {
    let ni = a.len() as i64;
    let nj = ni / 2;
    let ri = s * ni / 2 + r;
    let si = s * ni / 2 + i;
    let rj = r * ni / 2 + j;
    let sj = s * ni / 2 + j;
    let ij = i * ni + j;
    let ir = i * ni + r;
    let js = j * ni + s;
    let is = i * ni + s;
    let sr = s * ni + r;
    let rij = ri * ni + j;
    let rjs = rj * ni + s;
    let ris = ri * ni + s;
    let srs = sr * ni + s;
    let val_dist = dist[ij as usize];
    let val_a = a[ri as usize] - a[si as usize] + a[js as usize] - a[is as usize];
    let val_b = b[(p[s as usize] as usize) * (ni as usize) + p[i as usize] as usize] - b[(p[s as usize] as usize) * (ni as usize) + p[j as usize] as usize] + b[(p[r as usize] as usize) * (ni as usize) + p[j as usize] as usize] - b[(p[r as usize] as usize) * (ni as usize) + p[i as usize] as usize];
    let val_c = a[i as usize] - a[r as usize] + a[js as usize] - a[is as usize];
    let val_d = b[p[i as usize] as usize * (ni as usize) + p[s as usize] as usize] - b[p[j as usize] as usize * (ni as usize) + p[s as usize] as usize] + b[p[j as usize] as usize * (ni as usize) + p[r as usize] as usize] - b[p[i as usize] as usize * (ni as usize) + p[r as usize] as usize];
    mem::forget(ni);
    mem::forget(nj);
    val_dist + val_a * val_b + val_c * val_d
}

fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m[(i * (m.len() as i64 / 2) + j) as usize]
}

