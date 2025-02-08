
use std::cmp;
use std::f64;
use std::i64;
use std::mem;

static VERBOSE: bool = false;

const INF: i64 = i64::MAX;

fn qap_solve_ts(
    a: Box<Matrix>,
    b: Box<Matrix>,
    mut p: Vec<i64>,
    opt: i64,
    tabu_duration: i64,
    aspiration: i64,
    nr_iterations: i64,
) -> i64 {
    let mut i: i64;
    let mut j: i64;
    let mut current_cost: i64;
    let mut iter: i64;
    let mut best_cost: i64 = INF;
    let n: i64 = p.len() as i64;
    let mut dist: Box<Matrix> = Box::new(Matrix::new(n));
    let mut tabu_list: Box<Matrix> = Box::new(Matrix::new(n));
    let mut best_sol: Vec<i64> = p.clone();
    current_cost = 0;
    for i in 0..n {
        for j in 0..n {
            current_cost += a.get(i, j) * b.get(p[i as usize], p[j as usize]);
            if i < j {
                dist.set(i, j, delta(&a, &b, &p, i, j));
            }
        }
    }
    best_cost = current_cost;

    // tabu list initialization
    for i in 0..n {
        for j in 0..n {
            tabu_list.set(i, j, -(n * i + j));
        }
    }

    // tabu search loop
    for iter in 0..nr_iterations {
        if best_cost <= opt {
            break;
        }
        // find best move (i_retained, j_retained)
        let mut i_retained: i64 = INF; // in case all moves are tabu
        let mut j_retained: i64 = INF;
        let mut min_dist: i64 = INF;
        let mut already_aspired: bool = false;

        for i in 0..(n - 1) {
            for j in (i + 1)..n {
                let autorized: bool = tabu_list.get(i, p[j as usize]) < iter
                    || tabu_list.get(j, p[i as usize]) < iter;

                let aspired: bool = tabu_list.get(i, p[j as usize]) < iter - aspiration
                    || tabu_list.get(j, p[i as usize]) < iter - aspiration
                    || current_cost + dist.get(i, j) < best_cost;

                if (aspired && !already_aspired)
                    || (aspired && already_aspired && dist.get(i, j) < min_dist)
                    || (!aspired && !already_aspired && dist.get(i, j) < min_dist && autorized)
                {
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
            println!("All moves are tabu!"); // to be improved
        } else {
            // transpose elements in pos. i_retained and j_retained
            p.swap(i_retained as usize, j_retained as usize);

            // update solution value
            current_cost += dist.get(i_retained, j_retained);
            // forbid reverse move for a random number of iterations
            let z: i64 = iter + (cube(rand::random::<f64>()) * tabu_duration as f64) as i64;
            tabu_list.set(i_retained, p[j_retained as usize], z);
            let z: i64 = iter + (cube(rand::random::<f64>()) * tabu_duration as f64) as i64;
            tabu_list.set(j_retained, p[i_retained as usize], z);

            // best solution improved ?
            if current_cost < best_cost {
                best_cost = current_cost;
                best_sol = p.clone();
                if VERBOSE {
                    println!("iteration {}: cost={}", iter, best_cost);
                    print_vector(&best_sol);
                }
            }

            // update matrix of the move costs
            for i in 0..(n - 1) {
                for j in (i + 1)..n {
                    if i != i_retained
                        && i != j_retained
                        && j != i_retained
                        && j != j_retained
                    {
                        let y: i64 = delta_part(
                            &a,
                            &b,
                            &dist,
                            &p,
                            i,
                            j,
                            i_retained,
                            j_retained,
                        );
                        dist.set(i, j, y);
                    } else {
                        let y: i64 = delta(&a, &b, &p, i, j);
                        dist.set(i, j, y);
                    }
                }
            }
        }
    }
    p = best_sol;
    best_cost
}

fn len(v: &Vec<i64>) -> i64 {
    v.len() as i64
}

fn new_matrix(n: i64) -> Box<Matrix> {
    Box::new(Matrix::new(n))
}

fn copy_vector(v: &mut Vec<i64>, w: &Vec<i64>) {
    for i in 0..v.len() {
        v[i] = w[i];
    }
}

fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m.get(i, j)
}

fn delta(a: &Matrix, b: &Matrix, p: &Vec<i64>, r: i64, s: i64) -> i64 {
    let mut d: i64 = (a.get(r, r) - a.get(s, s)) * (b.get(p[s as usize], p[s as usize]) - b.get(p[r as usize], p[r as usize]))
        + (a.get(r, s) - a.get(s, r)) * (b.get(p[s as usize], p[r as usize]) - b.get(p[r as usize], p[s as usize]));
    let mut i: i64 = 0;
    while i < len(p) {
        if i != r && i != s {
            d += (a.get(i, r) - a.get(i, s)) * (b.get(p[i as usize], p[s as usize]) - b.get(p[i as usize], p[r as usize]))
                + (a.get(r, i) - a.get(s, i)) * (b.get(p[s as usize], p[i as usize]) - b.get(p[r as usize], p[i as usize]));
        }
        i += 1;
    }
    d
}

fn set(m: &mut Matrix, i: i64, j: i64, v: i64) {
    m.set(i, j, v);
}

fn swap(p: &mut Vec<i64>, i: usize, j: usize) {
    p.swap(i, j);
}

fn cube(x: f64) -> f64 {
    x * x * x
}

fn print_vector(v: &Vec<i64>) {
    for i in 0..v.len() {
        print!("{} ", v[i]);
    }
    println!();
}

fn delta_part(
    a: &Matrix,
    b: &Matrix,
    dist: &Matrix,
    p: &Vec<i64>,
    i: i64,
    j: i64,
    r: i64,
    s: i64,
) -> i64 {
    dist.get(i, j)
        + (a.get(r, i) - a.get(r, j) + a.get(s, j) - a.get(s, i))
            * (b.get(p[s as usize], p[i as usize])
                - b.get(p[s as usize], p[j as usize])
                + b.get(p[r as usize], p[j as usize])
                - b.get(p[r as usize], p[i as usize]))
        + (a.get(i, r) - a.get(j, r) + a.get(j, s) - a.get(i, s))
            * (b.get(p[i as usize], p[s as usize])
                - b.get(p[j as usize], p[s as usize])
                + b.get(p[j as usize], p[r as usize])
                - b.get(p[i as usize], p[r as usize]))
}

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn new(n: i64) -> Matrix {
        Matrix {
            n,
            a: vec![0; (n * n) as usize],
        }
    }

    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }

    fn set(&mut self, i: i64, j: i64, v: i64) {
        self.a[(i * self.n + j) as usize] = v;
    }
}
