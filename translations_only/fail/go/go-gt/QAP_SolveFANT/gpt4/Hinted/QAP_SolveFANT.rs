
use std::cmp;
use rand::Rng;

static mut VERBOSE: bool = false;
const INF: i64 = i64::MAX;

struct Vector(Vec<i64>);

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

fn qap_solve_fant(a: &Matrix, b: &Matrix, p: &mut Vector, r: i64, m: i64) -> i64 {
    let mut inc = 1;
    let n = p.0.len() as i64;
    let mut w = p.0.clone();
    let mut trace = new_matrix(n);
    init_trace(n, inc, &mut trace);
    let mut cc = INF;

    for i in 0..m {
        gen_trace(&mut w, &mut trace);
        let mut c = cost(a, b, &w);
        local_search(a, b, &mut w, &mut c);
        if c < cc {
            cc = c;
            p.0 = w.clone();
            unsafe {
                if VERBOSE {
                    println!("iteration {}: cost={}", i, cc);
                    p.print();
                }
            }
            inc = 1;
            init_trace(n, inc, &mut trace);
        } else {
            update_trace(n, &w, &p.0, &mut inc, r, &mut trace);
        }
    }
    cc
}

fn new_matrix(n: i64) -> Matrix {
    Matrix {
        n,
        a: vec![0; (n * n) as usize],
    }
}

fn init_trace(n: i64, inc: i64, trace: &mut Matrix) {
    for i in 0..n {
        for j in 0..n {
            trace.set(i, j, inc);
        }
    }
}

fn gen_trace(p: &mut Vec<i64>, trace: &mut Matrix) {
    let n = p.len() as i64;
    let mut nexti: Vec<i64> = (0..n).collect();
    let mut nextj: Vec<i64> = (0..n).collect();
    let mut sum_trace: Vec<i64> = vec![0; n as usize];

    perm(&mut nexti);
    perm(&mut nextj);
    for i in 0..n {
        for j in 0..n {
            sum_trace[i as usize] += trace.get(i, j);
        }
    }

    let mut rng = rand::thread_rng();
    for i in 0..n {
        let target = rng.gen_range(0..sum_trace[nexti[i as usize] as usize]);
        let mut j = 0;
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
    let mut rng = rand::thread_rng();
    for i in 0..n {
        let j = rng.gen_range(i..n);
        p.swap(i as usize, j as usize);
    }
}

impl Matrix {
    fn set(&mut self, i: i64, j: i64, v: i64) {
        let idx = (i * self.n + j) as usize;
        self.a[idx] = v;
    }

    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}

fn cost(a: &Matrix, b: &Matrix, p: &Vec<i64>) -> i64 {
    let mut c = 0;
    for i in 0..p.len() as i64 {
        for j in 0..p.len() as i64 {
            c += a.get(i, j) * b.get(p[i as usize], p[j as usize]);
        }
    }
    c
}

fn local_search(a: &Matrix, b: &Matrix, p: &mut Vec<i64>, cost: &mut i64) {
    let n = p.len() as i64;
    let mut move_vec: Vec<i64> = Vec::new();
    for i in 0..n-1 {
        for j in i+1..n {
            move_vec.push(n*i + j);
        }
    }
    let mut improved = true;
    while improved {
        improved = false;
        perm(&mut move_vec);
        for &move_val in &move_vec {
            let r = move_val / n;
            let s = move_val % n;
            let d = delta(a, b, p, r, s);
            if d < 0 {
                *cost += d;
                p.swap(r as usize, s as usize);
                improved = true;
            }
        }
    }
}

fn delta(a: &Matrix, b: &Matrix, p: &Vec<i64>, r: i64, s: i64) -> i64 {
    let mut d = 0;
    d += (a.get(r, r) - a.get(s, s)) * (b.get(p[s as usize], p[s as usize]) - b.get(p[r as usize], p[r as usize])) +
         (a.get(r, s) - a.get(s, r)) * (b.get(p[s as usize], p[r as usize]) - b.get(p[r as usize], p[s as usize]));
    for i in 0..p.len() as i64 {
        if i != r && i != s {
            d += (a.get(i, r) - a.get(i, s)) * (b.get(p[i as usize], p[s as usize]) - b.get(p[i as usize], p[r as usize])) +
                 (a.get(r, i) - a.get(s, i)) * (b.get(p[s as usize], p[i as usize]) - b.get(p[r as usize], p[i as usize]));
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

impl Vector {
    fn print(&self) {
        for &item in &self.0 {
            print!("{} ", item);
        }
        println!();
    }
}
