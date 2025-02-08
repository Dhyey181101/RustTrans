
use rand::Rng;

fn perm(p: &mut Vec<i64>) {
    let n = p.len() as i64;
    for i in 0..n {
        p[i as usize] = i;
    }
    for i in 0..n {
        let j = rand::thread_rng().gen_range(0..n);
        p.swap(i as usize, j as usize);
    }
}
