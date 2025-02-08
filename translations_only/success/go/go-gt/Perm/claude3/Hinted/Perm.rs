
use rand::Rng;

type Vector = Vec<i64>;

fn perm(p: &mut Vector) {
    let n = p.len() as i64;
    for i in 0..n {
        p[i as usize] = i;
    }
    for i in 0..n {
        let j = rand::thread_rng().gen_range(i..n);
        p.swap(i as usize, j as usize);
    }
}

fn swap(p: &mut Vector, i: usize, j: usize) {
    let x = p[i];
    p[i] = p[j];
    p[j] = x;
}
