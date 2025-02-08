
use std::{fmt::Display, vec};

const INF: i64 = i64::MAX;

struct Vector(Vec<i64>);

impl Vector {
    fn len(&self) -> i64 {
        self.0.len() as i64
    }
    
    fn copy(&mut self, w: &Vector) {
        self.0.copy_from_slice(&w.0);
    }
    
    fn swap(&mut self, i: usize, j: usize) {
        self.0.swap(i, j);
    }
    
    fn print(&self) {
        for i in &self.0 {
            print!("{} ", i);
        }
        println!("");
    }
}

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn new(n: i64) -> Self {
        Self {
            n,
            a: vec![0; (n * n) as usize],
        }
    }
    
    fn set(&mut self, i: i64, j: i64, v: i64) {
        self.a[(i * self.n + j) as usize] = v;
    }
    
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}

fn qap_solve_fant(a: &Matrix, b: &Matrix, mut p: Vector, r: i64, m: i64) -> i64 {
    let mut inc = 1;
    let mut i = 0;
    let mut c = 0;
    let n = p.len();
    let mut w = Vector(vec![0; n as usize]);
    w.copy(&p);
    let mut trace = Matrix::new(n);
    init_trace(n, inc, &mut trace);
    let mut cc = INF;

    while i < m {
        i += 1; 
        gen_trace(&mut w, &trace);
        c = cost(&a, &b, &w);
        local_search(&a, &b, &mut w, &mut c);
        if c < cc {
            cc = c;
            p.copy(&w);
            inc = 1;
            init_trace(n, inc, &mut trace);
        } else {
            update_trace(n, &w, &p, &mut inc, r, &mut trace);
        }
    }

    cc
}

fn init_trace(n: i64, inc: i64, trace: &mut Matrix) {
    for i in 0..n {
        for j in 0..n {
            trace.set(i, j, inc);
        }
    }
}

fn gen_trace(p: &mut Vector, trace: &Matrix) {
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
        let target = unif(0, sum_trace.0[nexti.0[i as usize] as usize] - 1);
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
        
        nextj.swap(j as usize, i as usize);
    }
}

fn perm(p: &mut Vector) {
    let n = p.len();
    for i in 0..n {
        p.0[i as usize] = i; 
    }
    
    for i in 0..n {
        p.swap(i as usize, (i + rand::random::<i64>() % (n - i)) as usize);
    }
}

fn unif(low: i64, high: i64) -> i64 {
    low + (rand::random::<f64>() * (high - low + 1) as f64) as i64
}

fn cost(a: &Matrix, b: &Matrix, p: &Vector) -> i64 {
    let mut c = 0;

    for i in 0..p.len() {
        for j in 0..p.len() {
            c += a.get(i, j) * b.get(p.0[i as usize], p.0[j as usize]);
        }
    }

    c
}

fn local_search(a: &Matrix, b: &Matrix, p: &mut Vector, cost: &mut i64) {
    let n = p.len();
    let mut move_vec = Vector(vec![0; (n * (n - 1) / 2) as usize]);

    let mut nm = 0;
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            move_vec.0[nm as usize] = n * i + j;
            nm += 1;
        }
    }

    let mut improved = true;
    for _k in 0..2 {
        if !improved {
            break;
        }

        improved = false;
        for i in 0..(nm - 1) {
            move_vec.swap(i as usize, unif(i + 1, nm - 1) as usize);
        }

        for i in 0..nm {
            let r = move_vec.0[i as usize] / n;
            let s = move_vec.0[i as usize] % n;
            let d = delta(&a, &b, p, r, s);
            if d < 0 {
                *cost += d;
                p.swap(r as usize, s as usize);
                improved = true;
            }
        }
    }
}

fn delta(a: &Matrix, b: &Matrix, p: &Vector, r: i64, s: i64) -> i64 {
    let mut d = (a.get(r, r) - a.get(s, s)) * (b.get(p.0[s as usize], p.0[s as usize]) - b.get(p.0[r as usize], p.0[r as usize]))
        + (a.get(r, s) - a.get(s, r)) * (b.get(p.0[s as usize], p.0[r as usize]) - b.get(p.0[r as usize], p.0[s as usize]));

    for i in 0..p.len() {
        if i != r && i != s {
            d += (a.get(i, r) - a.get(i, s)) * (b.get(p.0[i as usize], p.0[s as usize]) - b.get(p.0[i as usize], p.0[r as usize]))
                + (a.get(r, i) - a.get(s, i)) * (b.get(p.0[s as usize], p.0[i as usize]) - b.get(p.0[r as usize], p.0[i as usize]));
        }
    }

    d
}

fn update_trace(n: i64, p: &Vector, best_p: &Vector, inc: &mut i64, r: i64, trace: &mut Matrix) {
    let mut i = 0;
    while i < n && p.0[i as usize] == best_p.0[i as usize] {
        i += 1; 
    }

    if i == n {
        *inc += 1;
        init_trace(n, *inc, trace);
    } else {
        for i in 0..n {
            trace.set(i, p.0[i as usize], trace.get(i, p.0[i as usize]) + *inc);
            trace.set(i, best_p.0[i as usize], trace.get(i, best_p.0[i as usize]) + r);
        }
    }
}

