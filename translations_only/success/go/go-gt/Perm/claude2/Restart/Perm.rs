
use rand::Rng;

type Vector = Box<[i64]>;

fn perm(mut p: Vector) {
    let n = p.len() as i64;
    let mut i = 0_i64;
    while i < n {
        p[i as usize] = i;
        i += 1;
    }
    i = 0_i64;
    while i < n {
        let j = i + rand::thread_rng().gen_range(0..(n - i)) as i64;
        swap(&mut p, i, j);
        i += 1;
    }
}

fn swap(p: &mut Vector, i: i64, j: i64) {
    let i = i as usize;
    let j = j as usize;
    let tmp = p[i];
    p[i] = p[j];
    p[j] = tmp;
}

