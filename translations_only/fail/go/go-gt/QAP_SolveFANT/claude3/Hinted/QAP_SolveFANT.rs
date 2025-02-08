
use rand::Rng;
use std::cmp;

const INF: i64 = std::i64::MAX;

static mut VERBOSE: bool = false;

fn qap_solve_fant(a: Box<Matrix>, b: Box<Matrix>, mut p: Vec<i64>, r: i64, m: i64) -> i64 {
    let mut inc: i64 = 1;
    let n = p.len() as i64;
    let mut w = p.clone();
    let mut trace = Matrix::new(n);
    init_trace(n, inc, &mut trace);
    let mut cc = INF;

    // FANT iterations
    for i in 0..m {
        // Build a new solution
        gen_trace(&mut w, &trace);
        let mut c = cost(&a, &b, &w);
        // Improve solution with a local search
        local_search(&a, &b, &mut w, &mut c);
        // Best solution improved ?
        if c < cc {
            cc = c;
            p = w.clone();
            if unsafe { VERBOSE } {
                println!("iteration {}: cost={}", i, cc);
                print_vector(&p);
            }
            inc = 1;
            init_trace(n, inc, &mut trace);
        } else {
            // Memory update
            update_trace(n, &w, &p, &mut inc, r, &mut trace);
        }
    }
    cc
}

fn len(v: &Vec<i64>) -> i64 {
    v.len() as i64
}

fn copy(v: &mut Vec<i64>, w: &Vec<i64>) {
    for i in 0..v.len() {
        v[i] = w[i];
    }
}

fn init_trace(n: i64, inc: i64, trace: &mut Matrix) {
    for i in 0..n {
        for j in 0..n {
            trace.set(i, j, inc);
        }
    }
}

fn gen_trace(p: &mut Vec<i64>, trace: &Matrix) {
    let n = len(p) as i64;
    let mut nexti = vec![0; n as usize];
    let mut nextj = vec![0; n as usize];
    let mut sum_trace = vec![0; n as usize];

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

fn swap(p: &mut Vec<i64>, i: usize, j: usize) {
    let x = p[i];
    p[i] = p[j];
    p[j] = x;
}

fn unif(low: i64, high: i64) -> i64 {
    let mut rng = rand::thread_rng();
    low + (rng.gen::<f64>() * (high - low + 1) as f64) as i64
}

fn cost(a: &Matrix, b: &Matrix, p: &Vec<i64>) -> i64 {
    let mut c = 0;
    for i in 0..len(p) {
        for j in 0..len(p) {
            c += a.get(i, j) * b.get(p[i as usize], p[j as usize]);
        }
    }
    c
}

fn local_search(a: &Matrix, b: &Matrix, p: &mut Vec<i64>, cost: &mut i64) {
    let n = len(p);
    let mut move_vec = vec![0; (n * (n - 1) / 2) as usize];
    let mut n_mov = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            move_vec[n_mov as usize] = (n * i + j) as usize;
            n_mov += 1;
        }
    }
    let mut improved = true;
    for _ in 0..2 {
        if !improved {
            break;
        }
        improved = false;
        for i in 0..n_mov - 1 {
            move_vec.swap(i as usize, unif(i + 1, n_mov - 1) as usize);
        }
        for i in 0..n_mov {
            let r = move_vec[i as usize] / n as usize;
            let s = move_vec[i as usize] % n as usize;
            let d = delta(a, b, p, r as i64, s as i64);
            if d < 0 {
                *cost += d;
                swap(p, r, s);
                improved = true;
            }
        }
    }
}

fn delta(a: &Matrix, b: &Matrix, p: &Vec<i64>, r: i64, s: i64) -> i64 {
    let mut d = (a.get(r, r) - a.get(s, s)) * (b.get(p[s as usize], p[s as usize]) - b.get(p[r as usize], p[r as usize]))
        + (a.get(r, s) - a.get(s, r)) * (b.get(p[s as usize], p[r as usize]) - b.get(p[r as usize], p[s as usize]));
    for i in 0..len(&p) {
        if i != r && i != s {
            d += (a.get(i, r) - a.get(i, s)) * (b.get(p[i as usize], p[s as usize]) - b.get(p[i as usize], p[r as usize]))
                + (a.get(r, i) - a.get(s, i)) * (b.get(p[s as usize], p[i as usize]) - b.get(p[r as usize], p[i as usize]));
        }
    }
    d
}

fn update_trace(n: i64, p: &Vec<i64>, best_p: &Vec<i64>, inc: &mut i64, r: i64, trace: &mut Matrix) {
    let mut i = 0;
    while i < n && p[i as usize] == best_p[i as usize] {
        i += 1;
    }
    if i == n {
        *inc += 1;
        init_trace(n, *inc, trace);
    } else {
        for i in 0..n {
            trace.set(i, p[i as usize], trace.get(i, p[i as usize]) + *inc);
            trace.set(i, best_p[i as usize], trace.get(i, best_p[i as usize]) + r);
        }
    }
}

fn print_vector(v: &Vec<i64>) {
    for i in v {
        print!("{} ", i);
    }
    println!();
}

struct Matrix {
    n: i64,
    a: Box<[i64]>,
}

impl Matrix {
    fn new(n: i64) -> Matrix {
        Matrix {
            n,
            a: vec![0; (n * n) as usize].into_boxed_slice(),
        }
    }

    fn set(&mut self, i: i64, j: i64, v: i64) {
        self.a[(i * self.n + j) as usize] = v;
    }

    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}

