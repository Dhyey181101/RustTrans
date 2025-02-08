
use rand::Rng;

fn gen_trace(p: &mut Vec<i64>, trace: &Matrix) {
    let n = p.len() as i64;
    let mut nexti = vec![0; p.len()];
    let mut nextj = vec![0; p.len()];
    let mut sum_trace = vec![0; p.len()];

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
        p.swap(i as usize, (i + rng.gen_range(0..n - i)) as usize);
    }
}

fn unif(low: i64, high: i64) -> i64 {
    let mut rng = rand::thread_rng();
    low + (rng.gen::<f64>() * (high - low + 1) as f64) as i64
}

struct Matrix {
    n: i64,
    a: Box<Vec<i64>>,
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[((i * self.n) + j) as usize]
    }
}
