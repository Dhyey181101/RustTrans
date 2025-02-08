

use rand::Rng;

struct Vector(Vec<i64>);

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

fn gen_trace(mut p: Vector, trace: &Matrix) {
    let n = vector_len(&p);
    let mut nexti = Vector(vec![0; n as usize]);
    let mut nextj = Vector(vec![0; n as usize]);
    let mut sum_trace = Vector(vec![0; n as usize]);

    perm(&mut nexti.0);
    perm(&mut nextj.0);
    
    for i in 0..n {
        for j in 0..n {
            sum_trace.0[i as usize] += matrix_get(trace, i, nextj.0[j as usize]);
        }
    }

    for i in 0..n {
        let target = rand::thread_rng().gen_range(0..sum_trace.0[nexti.0[i as usize] as usize]);
        let mut j = i;
        let mut sum = matrix_get(trace, nexti.0[i as usize], nextj.0[j as usize]);
        while sum < target {
            j += 1;
            sum += matrix_get(trace, nexti.0[i as usize], nextj.0[j as usize]);
        }
        p.0[nexti.0[i as usize] as usize] = nextj.0[j as usize];
        for k in i..n {
            sum_trace.0[nexti.0[k as usize] as usize] -= matrix_get(trace, nexti.0[k as usize], nextj.0[j as usize]);
        }
        nextj.0.swap(j as usize, i as usize);
    }
}

fn perm(v: &mut Vec<i64>) {
    let n = v.len() as i64;
    for i in 0..n {
        v[i as usize] = i;
    }
    
    for i in 0..n {
        v.swap(i as usize, (i + rand_range(0, n - i)) as usize);
    }
}

fn vector_len(v: &Vector) -> i64 {
    v.0.len() as i64
}

fn matrix_get(m: &Matrix, i: i64, j: i64) -> i64 {
    m.a[(i * m.n + j) as usize]
}

fn rand_range(low: i64, high: i64) -> i64 {
    low + rand::thread_rng().gen_range(0..(high - low + 1)) as i64  
}

