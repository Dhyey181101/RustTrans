
use rand::Rng;

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}

fn gen_trace(p: &mut Vec<i64>, trace: &Matrix) {
    let n = p.len() as i64;
    let mut nexti: Vec<i64> = vec![0; n as usize];
    let mut nextj: Vec<i64> = vec![0; n as usize];
    let mut sum_trace: Vec<i64> = vec![0; n as usize];

    perm(&mut nexti);
    perm(&mut nextj);
    for i in 0..n {
        for j in 0..n {
            sum_trace[i as usize] += trace.get(i, j);
        }
    }

    for i in 0..n {
        let target = unif(0, sum_trace[nexti[i as usize] as usize] - 1);
        let mut j = i;
        let mut sum = trace.get(nexti[i as usize], nextj[j as usize]);
        while sum < target {
            j += 1;
            sum += trace.get(nexti[i as usize], nextj[j as usize]);
        }
        p[nexti[i as usize] as usize] = nextj[j as usize];
        for k in i..n {
            sum_trace[nexti[k as usize] as usize] -= trace.get(nexti[k as usize], nextj[j as usize]);
        }
        nextj.swap(j as usize, i as usize);
    }
}

fn perm(p: &mut Vec<i64>) {
    let n = p.len() as i64;
    for i in 0..n {
        p[i as usize] = i;
    }
    let mut rng = rand::thread_rng();
    for i in 0..n {
        let j = i + rng.gen_range(0..n - i);
        p.swap(i as usize, j as usize);
    }
}

fn unif(low: i64, high: i64) -> i64 {
    let mut rng = rand::thread_rng();
    low + rng.gen_range(0..=high - low)
}
