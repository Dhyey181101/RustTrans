
use std::fmt;
use rand::Rng;

static mut VERBOSE: bool = false;
const INF: i64 = i64::MAX;

struct Vector(Vec<i64>);

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

fn qap_solve_ts(a: &Matrix, b: &Matrix, p: &mut Vector, opt: i64, tabu_duration: i64, aspiration: i64, nr_iterations: i64) -> i64 {
    let mut best_cost = INF;
    let n = p.len() as i64;
    let mut dist = new_matrix(n);
    let mut tabu_list = new_matrix(n);
    let mut best_sol = p.0.clone();
    let mut current_cost = 0;
    for i in 0..n {
        for j in 0..n {
            current_cost += a.get(i, j) * b.get(p.0[i as usize] as i64, p.0[j as usize] as i64);
            if i < j {
                dist.set(i, j, delta(a, b, &p, i, j));
            }
        }
    }
    best_cost = current_cost;

    for i in 0..n {
        for j in 0..n {
            tabu_list.set(i, j, -(n * i + j));
        }
    }

    let mut iter = 0;
    while iter < nr_iterations && best_cost > opt {
        let mut i_retained = INF;
        let mut j_retained = INF;
        let mut min_dist = INF;
        let mut already_aspired = false;

        for i in 0..n - 1 {
            for j in i + 1..n {
                let authorized = tabu_list.get(i, p.0[j as usize] as i64) < iter || tabu_list.get(j, p.0[i as usize] as i64) < iter;
                let aspired = tabu_list.get(i, p.0[j as usize] as i64) < iter - aspiration || tabu_list.get(j, p.0[i as usize] as i64) < iter - aspiration || current_cost + dist.get(i, j) < best_cost;

                if (aspired && !already_aspired) || (aspired && already_aspired && dist.get(i, j) < min_dist) || (!aspired && !already_aspired && dist.get(i, j) < min_dist && authorized) {
                    i_retained = i;
                    j_retained = j;
                    min_dist = dist.get(i, j);
                    if aspired {
                        already_aspired = true;
                    }
                }
            }
        }

        if i_retained == INF {
            println!("All moves are tabu!");
        } else {
            p.swap(i_retained as usize, j_retained as usize);
            current_cost += dist.get(i_retained, j_retained);
            let mut rng = rand::thread_rng();
            let z = iter + (cube(rng.gen::<f64>()) * tabu_duration as f64) as i64;
            tabu_list.set(i_retained, p.0[j_retained as usize], z);
            let z = iter + (cube(rng.gen::<f64>()) * tabu_duration as f64) as i64;
            tabu_list.set(j_retained, p.0[i_retained as usize], z);

            if current_cost < best_cost {
                best_cost = current_cost;
                best_sol = p.0.clone();
                unsafe {
                    if VERBOSE {
                        println!("iteration {}: cost={}", iter, best_cost);
                        print_vector(&best_sol);
                    }
                }
            }

            for i in 0..n - 1 {
                for j in i + 1..n {
                    let y = if i != i_retained && i != j_retained && j != i_retained && j != j_retained {
                        delta_part(a, b, &dist, &p, i, j, i_retained, j_retained)
                    } else {
                        delta(a, b, &p, i, j)
                    };
                    dist.set(i, j, y);
                }
            }
        }
        iter += 1;
    }
    p.0 = best_sol;
    best_cost
}

fn new_matrix(n: i64) -> Matrix {
    Matrix { n, a: vec![0; (n * n) as usize] }
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }

    fn set(&mut self, i: i64, j: i64, v: i64) {
        self.a[(i * self.n + j) as usize] = v;
    }
}

impl Vector {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.0.swap(i, j);
    }
}

fn delta(a: &Matrix, b: &Matrix, p: &Vector, r: i64, s: i64) -> i64 {
    let mut d = (a.get(r, r) - a.get(s, s)) * (b.get(p.0[s as usize], p.0[s as usize]) - b.get(p.0[r as usize], p.0[r as usize])) + (a.get(r, s) - a.get(s, r)) * (b.get(p.0[s as usize], p.0[r as usize]) - b.get(p.0[r as usize], p.0[s as usize]));
    for i in 0..p.len() as i64 {
        if i != r && i != s {
            d += (a.get(i, r) - a.get(i, s)) * (b.get(p.0[i as usize], p.0[s as usize]) - b.get(p.0[i as usize], p.0[r as usize])) + (a.get(r, i) - a.get(s, i)) * (b.get(p.0[s as usize], p.0[i as usize]) - b.get(p.0[r as usize], p.0[i as usize]));
        }
    }
    d
}

fn delta_part(a: &Matrix, b: &Matrix, dist: &Matrix, p: &Vector, i: i64, j: i64, r: i64, s: i64) -> i64 {
    dist.get(i, j) + (a.get(r, i) - a.get(r, j) + a.get(s, j) - a.get(s, i)) * (b.get(p.0[s as usize], p.0[i as usize]) - b.get(p.0[s as usize], p.0[j as usize]) + b.get(p.0[r as usize], p.0[j as usize]) - b.get(p.0[r as usize], p.0[i as usize])) + (a.get(i, r) - a.get(j, r) + a.get(j, s) - a.get(i, s)) * (b.get(p.0[i as usize], p.0[s as usize]) - b.get(p.0[j as usize], p.0[s as usize]) + b.get(p.0[j as usize], p.0[r as usize]) - b.get(p.0[i as usize], p.0[r as usize]))
}

fn cube(x: f64) -> f64 {
    x * x * x
}

fn print_vector(v: &Vec<i64>) {
    for &val in v {
        print!("{} ", val);
    }
    println!();
}
