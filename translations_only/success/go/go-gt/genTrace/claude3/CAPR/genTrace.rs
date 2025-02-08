

use rand::Rng;

fn gen_trace(p: &mut Vec<i64>, trace: &Matrix) {
    let n = p.len() as i64;
    let mut nexti = vec![0; n as usize];
    let mut nextj = vec![0; n as usize];
    let mut sum_trace = vec![0; n as usize];

    perm(&mut nexti);
    perm(&mut nextj);
    for i in 0..n {
        for j in 0..n {
            sum_trace[i as usize] += get(trace, i, j);
        }
    }

    for i in 0..n {
        let target = unif(0, sum_trace[nexti[i as usize] as usize] - 1);
        let mut j = i;
        let mut sum = get(trace, nexti[i as usize], nextj[j as usize]);
        while sum < target {
            j += 1;
            sum += get(trace, nexti[i as usize], nextj[j as usize]);
        }
        p[nexti[i as usize] as usize] = nextj[j as usize];
        for k in i..n {
            sum_trace[nexti[k as usize] as usize] -= get(trace, nexti[k as usize], nextj[j as usize]);
        }
        swap(&mut nextj, j as usize, i as usize);
    }
}

fn perm(p: &mut Vec<i64>) {
    let n = p.len() as i64;
    for i in 0..n {
        p[i as usize] = i;
    }
    for i in 0..n {
        swap(p, i as usize, (i + rand::thread_rng().gen_range(0..n - i)) as usize);
    }
}

fn unif(low: i64, high: i64) -> i64 {
    low + (rand::thread_rng().gen_range(0.0..1.0) * (high - low + 1) as f64) as i64
}

struct Matrix {
    n: i64,
    a: Box<Vec<i64>>,
}

fn get(matrix: &Matrix, i: i64, j: i64) -> i64 {
    matrix.a[(i * matrix.n + j) as usize]
}

fn swap<T>(v: &mut Vec<T>, i: usize, j: usize) {
    v.swap(i, j);
}

