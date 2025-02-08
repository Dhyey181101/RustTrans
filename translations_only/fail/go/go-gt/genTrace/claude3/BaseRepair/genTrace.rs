
use rand::Rng;

fn gen_trace(p: &mut Vec<i64>, trace: &Matrix) {
    let n = p.len();
    let mut nexti = vec![0; n];
    let mut nextj = vec![0; n];
    let mut sum_trace = vec![0; n];

    perm(&mut nexti);
    perm(&mut nextj);

    for i in 0..n {
        for j in 0..n {
            sum_trace[i] += trace.a[(i * trace.n + j)];
        }
    }

    for i in 0..n {
        let target = unif(0, sum_trace[nexti[i]] - 1);
        let mut j = i;
        let mut sum = trace.a[(nexti[i] * trace.n + nextj[j])];
        while sum < target {
            j += 1;
            sum += trace.a[(nexti[i] * trace.n + nextj[j])];
        }
        p[nexti[i]] = nextj[j] as i64;
        for k in i..n {
            sum_trace[nexti[k]] -= trace.a[(nexti[k] * trace.n + nextj[j])];
        }
        swap_vec(&mut nextj, j, i);
    }
}

fn perm(p: &mut Vec<usize>) {
    let n = p.len();
    for i in 0..n {
        p[i] = i;
    }
    for i in 0..n {
        swap_vec(p, i, (i + rand::thread_rng().gen_range(0..n - i)));
    }
}

fn unif(low: i64, high: i64) -> i64 {
    low + (rand::thread_rng().gen_range(0.0..1.0) * (high - low + 1) as f64) as i64
}

fn swap_vec(v: &mut Vec<usize>, i: usize, j: usize) {
    let temp = v[i];
    v[i] = v[j];
    v[j] = temp;
}

struct Matrix {
    n: usize,
    a: Box<[i64]>,
}
