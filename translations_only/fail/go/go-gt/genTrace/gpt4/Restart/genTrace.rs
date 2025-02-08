
use rand::Rng;

struct Vector(Vec<i64>);

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

fn gen_trace(p: &mut Vector, trace: &Matrix) {
    let mut rng = rand::thread_rng();
    let n = p.len();
    let mut nexti = Vector(vec![0; n as usize]);
    let mut nextj = Vector(vec![0; n as usize]);
    let mut sum_trace = Vector(vec![0; n as usize]);

    perm(&mut nexti);
    perm(&mut nextj);
    for i in 0..n {
        for j in 0..n {
            sum_trace.0[i as usize] += trace.get(i, j);
        }
    }

    for i in 0..n {
        let target = unif(0, sum_trace.0[nexti.0[i as usize] as usize] - 1, &mut rng);
        let mut j = i;
        let mut sum = trace.get(nexti.0[i as usize], nextj.0[j as usize]);
        while sum < target {
            j += 1;
            sum += trace.get(nexti.0[i as usize], nextj.0[j as usize]);
        }
        p.0[nexti.0[i as usize] as usize] = nextj.0[j as usize];
        for k in i..n {
            sum_trace.0[nexti.0[k as usize] as usize] -= trace.get(nexti.0[k as usize], nextj.0[j as usize]);
        }
        nextj.swap(j, i);
    }
}

fn perm(p: &mut Vector) {
    let n = p.0.len() as i64;
    let mut rng = rand::thread_rng();
    for i in 0..n {
        p.0[i as usize] = i;
    }
    for i in 0..n {
        let j = i + rng.gen_range(0..n - i);
        p.swap(i, j);
    }
}

impl Vector {
    fn len(&self) -> i64 {
        self.0.len() as i64
    }

    fn swap(&mut self, i: i64, j: i64) {
        self.0.swap(i as usize, j as usize);
    }
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}

fn unif(low: i64, high: i64, rng: &mut rand::rngs::ThreadRng) -> i64 {
    rng.gen_range(low..=high)
}
