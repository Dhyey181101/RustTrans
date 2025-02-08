
use rand::Rng;

type Vector = Box<[i64]>;

fn perm(p: &mut Vector) {
    let n = p.len() as i64;
    let mut i: i64 = 0;
    while i < n {
        p[i as usize] = i;
        i += 1;
    }
    i = 0;
    while i < n {
        let j = rand::thread_rng().gen_range(i..n);
        p.swap(i as usize, j as usize);
        i += 1;
    }
}

fn swap(p: &mut Vector, i: usize, j: usize) {
    let x = p[i];
    p[i] = p[j];
    p[j] = x;
}
