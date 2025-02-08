

use rand::Rng;
use std::i64;

pub struct Vector([i64; 1000]);

pub struct Matrix {
    n: i64,
    a: Box<[i64]>,
}

pub fn QAP_SolveTS(
    a: &Matrix,
    b: &Matrix,
    p: &mut Vector,
    opt: i64,
    tabu_duration: i64,
    aspiration: i64,
    nr_iterations: i64,
) -> i64 {
    let n = p.0.len() as i64;
    let mut dist = Matrix::new(n);
    let mut tabu_list = Matrix::new(n);
    let mut best_sol = Vector([0; 1000]);
    let mut current_cost = 0;

    for i in 0..n {
        for j in 0..n {
            current_cost += a.a[i as usize * n as usize + j as usize] * b.a[p.0[i as usize] as usize * n as usize + p.0[j as usize] as usize];
            if i < j {
                dist.a[i as usize * n as usize + j as usize] = delta(a, b, p, i, j);
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            tabu_list.a[i as usize * n as usize + j as usize] = -(n * i + j);
        }
    }

    let mut iter = 0;
    let mut best_cost = i64::MAX;
    let mut rng = rand::thread_rng();
    while iter < nr_iterations && best_cost > opt {
        let mut i_retained = i64::MAX;
        let mut j_retained = i64::MAX;
        let mut min_dist = i64::MAX;
        let mut already_aspired = false;

        for i in 0..n - 1 {
            for j in i + 1..n {
                let autorized = (tabu_list.a[i as usize * n as usize + p.0[j as usize] as usize] < iter)
                    || (tabu_list.a[j as usize * n as usize + p.0[i as usize] as usize] < iter);

                let aspired = (tabu_list.a[i as usize * n as usize + p.0[j as usize] as usize] < iter - aspiration)
                    || (tabu_list.a[j as usize * n as usize + p.0[i as usize] as usize] < iter - aspiration)
                    || (current_cost + dist.a[i as usize * n as usize + j as usize] < best_cost);

                if (aspired && !already_aspired)
                    || (aspired && already_aspired && dist.a[i as usize * n as usize + j as usize] < min_dist)
                    || (!aspired && !already_aspired && dist.a[i as usize * n as usize + j as usize] < min_dist && autorized)
                {
                    i_retained = i;
                    j_retained = j;
                    min_dist = dist.a[i as usize * n as usize + j as usize];
                    if aspired {
                        already_aspired = true;
                    }
                }
            }
        }

        if i_retained == i64::MAX {
            println!("All moves are tabu!");
        } else {
            best_cost = current_cost;
            best_sol.0.swap(i_retained as usize, j_retained as usize);
            current_cost += dist.a[i_retained as usize * n as usize + j_retained as usize];

            let z = iter + (3_i64.pow(rng.gen::<u32>() as u32) * tabu_duration) as i64;
            tabu_list.a[i_retained as usize * n as usize + p.0[j_retained as usize] as usize] = z;
            tabu_list.a[j_retained as usize * n as usize + p.0[i_retained as usize] as usize] = z;

            if current_cost < best_cost {
                best_cost = current_cost;
                for i in 0..n {
                    p.0[i as usize] = best_sol.0[i as usize];
                }
                if true {
                    println!("iteration {}: cost={}", iter, best_cost);
                    for i in 0..n {
                        print!("{} ", p.0[i as usize]);
                    }
                    println!();
                }
            }

            for i in 0..n - 1 {
                for j in i + 1..n {
                    if i != i_retained && i != j_retained && j != i_retained && j != j_retained {
                        dist.a[i as usize * n as usize + j as usize] =
                            delta_part(&dist, a, b, p, i, j, i_retained, j_retained);
                    } else {
                        dist.a[i as usize * n as usize + j as usize] = delta(a, b, p, i, j);
                    }
                }
            }
        }
        iter += 1;
    }

    for i in 0..n {
        p.0[i as usize] = best_sol.0[i as usize];
    }

    best_cost
}

impl Vector {
    fn len(&self) -> i64 {
        self.0.len() as i64
    }
}

impl Matrix {
    fn new(n: i64) -> Self {
        Matrix {
            n,
            a: vec![0; (n * n) as usize].into_boxed_slice(),
        }
    }

    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[i as usize * self.n as usize + j as usize]
    }

    fn set(&mut self, i: i64, j: i64, v: i64) {
        self.a[i as usize * self.n as usize + j as usize] = v;
    }
}

fn delta(a: &Matrix, b: &Matrix, p: &Vector, r: i64, s: i64) -> i64 {
    let mut d = (a.get(r, r) - a.get(s, s)) * (b.get(p.0[s as usize], p.0[s as usize]) - b.get(p.0[r as usize], p.0[r as usize]));
    d += (a.get(r, s) - a.get(s, r)) * (b.get(p.0[s as usize], p.0[r as usize]) - b.get(p.0[r as usize], p.0[s as usize]));

    for i in 0..p.len() {
        if i != r && i != s {
            d += (a.get(i, r) - a.get(i, s)) * (b.get(p.0[i as usize], p.0[s as usize]) - b.get(p.0[i as usize], p.0[r as usize]));
            d += (a.get(r, i) - a.get(s, i)) * (b.get(p.0[s as usize], p.0[i as usize]) - b.get(p.0[r as usize], p.0[i as usize]));
        }
    }

    d
}

fn delta_part(
    dist: &Matrix,
    a: &Matrix,
    b: &Matrix,
    p: &Vector,
    i: i64,
    j: i64,
    r: i64,
    s: i64,
) -> i64 {
    (dist.a[i as usize * p.len() as usize + j as usize] + (a.get(r, i) - a.get(r, j) + a.get(s, j) - a.get(s, i)) * (b.get(p.0[s as usize], p.0[i as usize]) - b.get(p.0[s as usize], p.0[j as usize]) + b.get(p.0[r as usize], p.0[j as usize]) - b.get(p.0[r as usize], p.0[i as usize]))) +
        (a.get(i, r) - a.get(j, r) + a.get(j, s) - a.get(i, s)) *
        (b.get(p.0[i as usize], p.0[s as usize]) - b.get(p.0[j as usize], p.0[s as usize]) + b.get(p.0[j as usize], p.0[r as usize]) - b.get(p.0[i as usize], p.0[r as usize]))
}

