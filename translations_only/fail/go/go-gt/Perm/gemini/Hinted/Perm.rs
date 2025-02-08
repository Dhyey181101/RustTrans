
use rand::Rng;

fn perm(p: &mut Vec<i64>) {
    let n = p.len() as i64;
    let mut i = 0;
    while i < n {
        p[i as usize] = i;
        i += 1;
    }
    i = 0;
    while i < n {
        let j = i + rand::thread_rng().gen_range(0..n - i);
        p.swap(i as usize, j as usize);
        i += 1;
    }
}
