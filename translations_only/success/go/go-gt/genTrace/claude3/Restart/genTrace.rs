

use rand::Rng;

fn gen_trace(p: &mut Vec<i64>, trace: &Box<Matrix>) {
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
        let target = unif(0, sum_trace[nexti[i as usize]] - 1);
        let mut j = i;
        let mut sum = get(trace, nexti[i as usize] as i64, nextj[j as usize] as i64);
        while sum < target {
            j += 1;
            sum += get(trace, nexti[i as usize] as i64, nextj[j as usize] as i64);
        }
        p[nexti[i as usize] as usize] = nextj[j as usize] as i64;
        for k in i..n {
            sum_trace[nexti[k as usize] as usize] -= get(trace, nexti[k as usize] as i64, nextj[j as usize] as i64);
        }
        swap(&mut nextj, j as usize, i as usize);
    }
}

fn perm(p: &mut Vec<usize>) {
    let n = p.len();
    for i in 0..n {
        p[i] = i;
    }
    for i in 0..n {
        swap(p, i, (i + rand::thread_rng().gen_range(0..n - i)));
    }
}

fn unif(low: i64, high: i64) -> i64 {
    low + (rand::thread_rng().gen_range(0.0..1.0) * (high - low + 1) as f64) as i64
}

struct Matrix {
    n: i64,
    a: Box<Vec<i64>>,
}

fn get(matrix: &Box<Matrix>, i: i64, j: i64) -> i64 {
    matrix.a[(i * matrix.n + j) as usize]
}

fn swap(v: &mut Vec<usize>, i: usize, j: usize) {
    v.swap(i, j);
}

