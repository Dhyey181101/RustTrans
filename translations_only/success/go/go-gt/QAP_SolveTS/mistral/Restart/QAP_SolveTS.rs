




use rand::prelude::*;
use std::ops::Index;

pub struct Vector([i64; 1000]);

pub struct Matrix {
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

    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[i as usize * self.n as usize + j as usize]
    }

    fn set(&mut self, i: i64, j: i64, v: i64) {
        self.a[i as usize * self.n as usize + j as usize] = v;
    }
}

const INF: i64 = i64::MAX;

fn qap_solve_ts(
    a: &Matrix,
    b: &Matrix,
    p: &mut Vector,
    opt: i64,
    tabu_duration: i64,
    aspiration: i64,
    nr_iterations: i64,
) -> i64 {
    let n = p.0.len();
    let mut dist = Matrix::new(n as i64);
    let mut tabu_list = Matrix::new(n as i64);
    let mut best_sol = Vector([0; 1000]);
    best_sol.0.copy_from_slice(&p.0);
    let mut current_cost = 0;

    for i in 0..n {
        for j in 0..n {
            current_cost += a.get(i as i64, j as i64) * b.get(p.0[i as usize], p.0[j as usize]);
            if i < j {
                dist.set(
                    i as i64,
                    j as i64,
                    delta(a, b, p, i as i64, j as i64),
                );
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            tabu_list.set(i as i64, j as i64, -(n as i64 * i as i64 + j as i64));
        }
    }

    let mut iter = 0;
    let mut best_cost = INF;

    while iter < nr_iterations && best_cost > opt {
        let mut i_retained = INF;
        let mut j_retained = INF;
        let mut min_dist = INF;
        let mut already_aspired = false;

        for i in 0..n - 1 {
            for j in i + 1..n {
                let autorized = (tabu_list.get(i as i64, p.0[j as usize]) < iter)
                    || (tabu_list.get(j as i64, p.0[i as usize]) < iter);

                let aspired = (tabu_list.get(i as i64, p.0[j as usize]) < iter - aspiration)
                    || (tabu_list.get(j as i64, p.0[i as usize]) < iter - aspiration)
                    || (current_cost + dist.get(i as i64, j as i64) < best_cost);

                if (aspired && !already_aspired)
                    || (aspired && already_aspired && dist.get(i as i64, j as i64) < min_dist)
                    || (!aspired && !already_aspired && dist.get(i as i64, j as i64) < min_dist && autorized)
                {
                    i_retained = i as i64;
                    j_retained = j as i64;
                    min_dist = dist.get(i as i64, j as i64);
                    if aspired {
                        already_aspired = true;
                    }
                }
            }
        }

        if i_retained == INF {
            println!("All moves are tabu!");
        } else {
            best_cost = current_cost;
            best_sol.0.swap(i_retained as usize, j_retained as usize);

            current_cost += dist.get(i_retained, j_retained);
            let mut rng = rand::thread_rng();
            let z =
                iter + (1..).map(|_| rng.gen::<f64>() as i64).sum::<i64>() * tabu_duration;
            tabu_list.set(i_retained, p.0[j_retained as usize], z);
            tabu_list.set(j_retained, p.0[i_retained as usize], z);

            if current_cost < best_cost {
                best_cost = current_cost;

                for i in 0..n - 1 {
                    for j in i + 1..n {
                        if i != i_retained as usize
                            && i != j_retained as usize
                            && j != i_retained as usize
                            && j != j_retained as usize
                        {
                            dist.set(
                                i as i64,
                                j as i64,
                                delta_part(
                                    a,
                                    b,
                                    &dist,
                                    p,
                                    i as i64,
                                    j as i64,
                                    i_retained,
                                    j_retained,
                                ),
                            );
                        } else {
                            dist.set(
                                i as i64,
                                j as i64,
                                delta(a, b, p, i as i64, j as i64),
                            );
                        }
                    }
                }
            }
        }

        iter += 1;
    }

    for i in 0..n {
        p.0[i] = best_sol.0[i];
    }

    best_cost
}

fn delta(a: &Matrix, b: &Matrix, p: &Vector, r: i64, s: i64) -> i64 {
    let mut d = (a.get(r, r) - a.get(s, s)) * (b.get(p.0[s as usize], p.0[s as usize]) - b.get(p.0[r as usize], p.0[r as usize]));
    d += (a.get(r, s) - a.get(s, r)) * (b.get(p.0[s as usize], p.0[r as usize]) - b.get(p.0[r as usize], p.0[s as usize]));

    for i in 0..p.0.len() {
        if i != r as usize && i != s as usize {
            d += (a.get(i as i64, r) - a.get(i as i64, s)) * (b.get(p.0[i as usize], p.0[s as usize]) - b.get(p.0[i as usize], p.0[r as usize]));
            d += (a.get(r, i as i64) - a.get(s, i as i64)) * (b.get(p.0[s as usize], p.0[i as usize]) - b.get(p.0[r as usize], p.0[i as usize]));
        }
    }

    d
}

fn delta_part(
    a: &Matrix,
    b: &Matrix,
    dist: &Matrix,
    p: &Vector,
    i: i64,
    j: i64,
    r: i64,
    s: i64,
) -> i64 {
    dist.get(i, j) + (a.get(r, i) - a.get(r, j) + a.get(s, j) - a.get(s, i)) * (b.get(p.0[s as usize], p.0[i as usize]) - b.get(p.0[s as usize], p.0[j as usize]) + b.get(p.0[r as usize], p.0[j as usize]) - b.get(p.0[r as usize], p.0[i as usize])) +
    (a.get(i, r) - a.get(j, r) + a.get(j, s) - a.get(i, s)) * (b.get(p.0[i as usize], p.0[s as usize]) - b.get(p.0[j as usize], p.0[s as usize]) + b.get(p.0[j as usize], p.0[r as usize]) - b.get(p.0[i as usize], p.0[r as usize]))
}




