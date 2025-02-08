
use rand::Rng;

type Vector = Vec<i64>;

fn perm(p: &mut Vector) {
    let n = p.len() as i64;
    for i in 0..n {
        p[i as usize] = i;
    }
    for i in 0..n {
        swap(p, i, i + rand::thread_rng().gen_range(0..n - i));
    }
}

fn swap(p: &mut Vector, i: i64, j: i64) {
    let x = p[i as usize];
    p[i as usize] = p[j as usize];
    p[j as usize] = x;
}
