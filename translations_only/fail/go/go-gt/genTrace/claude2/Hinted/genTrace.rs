
use rand::Rng; 

struct Vector(Vec<i64>);

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

fn gen_trace(mut p: Vector, trace: &Matrix) {
    let n = p.0.len() as i64;
    let mut nexti = Vector(vec![0; n as usize]);
    let mut nextj = Vector(vec![0; n as usize]);
    let mut sum_trace = Vector(vec![0; n as usize]);

    perm(&mut nexti);
    perm(&mut nextj);
    
    for i in 0..n {
        for j in 0..n {
            sum_trace.0[i as usize] += trace.a[i as usize * trace.n as usize + j as usize];
        }
    }

    for i in 0..n {
        let target = rand::thread_rng().gen_range(0..sum_trace.0[nexti.0[i as usize] as usize]);
        let mut j = i;
        let mut sum = trace.a[nexti.0[i as usize] as usize * trace.n as usize + nextj.0[j as usize] as usize];
        while sum < target {
            j += 1;
            sum += trace.a[nexti.0[i as usize] as usize * trace.n as usize + nextj.0[j as usize] as usize];
        }
        p.0[nexti.0[i as usize] as usize] = nextj.0[j as usize];
        for k in i..n {
            sum_trace.0[nexti.0[k as usize] as usize] -= trace.a[nexti.0[k as usize] as usize * trace.n as usize + nextj.0[j as usize] as usize];
        }
        nextj.0.swap(j as usize, i as usize);
    }
}

impl Vector {
    fn len(&self) -> i64 {
        self.0.len() as i64
    }
}

fn perm(v: &mut Vector) {
    let n = v.0.len() as i64;
    for i in 0..n {
        v.0[i as usize] = i;
    }
    for i in 0..n {
        v.0.swap(i as usize, (i + rand::thread_rng().gen_range(0..(n - i))) as usize);
    }
} 

impl Vector {
    fn swap(&mut self, i: usize, j: usize) {
        self.0.swap(i, j);
    }
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[i as usize * self.n as usize + j as usize]
    }
}

fn unif(low: i64, high: i64) -> i64 {
    low + rand::thread_rng().gen_range(0..(high - low + 1)) as i64
}
