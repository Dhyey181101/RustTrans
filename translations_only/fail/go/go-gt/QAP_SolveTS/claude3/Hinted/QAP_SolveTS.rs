

use std::cmp;
use std::f64;
use std::i64;
use std::ops::{Index, IndexMut};
use rand::Rng;

static VERBOSE: bool = false;

const INF: i64 = i64::MAX;

fn qap_solve_ts(a: Box<Matrix>, b: Box<Matrix>, mut p: Vector, opt: i64, tabu_duration: i64, aspiration: i64, nr_iterations: i64) -> i64 {
    let mut i: i64;
    let mut j: i64;
    let mut current_cost: i64;
    let mut iter: i64;
    let mut best_cost: i64 = INF;
    let n: usize = p.len();
    let mut dist: Box<Matrix> = Box::new(Matrix::new(n as i64));
    let mut tabu_list: Box<Matrix> = Box::new(Matrix::new(n as i64));
    let mut best_sol: Vector = p.clone();
    current_cost = 0;
    for i in 0..n as i64 {
        for j in 0..n as i64 {
            current_cost += get(&a, i, j) * get(&b, p[i as usize], p[j as usize]);
            if i < j {
                set(&mut *dist, i, j, delta(&a, &b, &p, i, j));
            }
        }
    }
    best_cost = current_cost;

    // tabu list initialization
    for i in 0..n as i64 {
        for j in 0..n as i64 {
            set(&mut *tabu_list, i, j, -(n as i64 * i + j));
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

        for i in 0..(n as i64 - 1) {
            for j in (i + 1)..(n as i64) {
                let autorized: bool = (get(&tabu_list, i, p[j as usize]) < iter) ||
                    (get(&tabu_list, j, p[i as usize]) < iter);

                let aspired: bool =
                    (get(&tabu_list, i, p[j as usize]) < iter - aspiration) ||
                        (get(&tabu_list, j, p[i as usize]) < iter - aspiration) ||
                        (current_cost + get(&dist, i, j) < best_cost);

                if (aspired && !already_aspired) || // first move aspired
                    (aspired && already_aspired && // many move aspired
                        (get(&dist, i, j) < min_dist)) || // => take best one
                    (!aspired && !already_aspired && // no move aspired yet
                        (get(&dist, i, j) < min_dist) && autorized) {
                    i_retained = i;
                    j_retained = j;
                    min_dist = get(&dist, i, j);
                    if aspired {
                        already_aspired = true;
                    }
                }
            }
        }

        if i_retained == INF {
            println!("All moves are tabu!"); // to be improved
        } else { // transpose elements in pos. i_retained and j_retained

            swap(&mut p, i_retained, j_retained);

            // update solution value
            current_cost += get(&dist, i_retained, j_retained);
            // forbid reverse move for a random number of iterations
            let z: i64 = iter + (cube(rand::thread_rng().gen::<f64>()) * tabu_duration as f64) as i64;
            set(&mut *tabu_list, i_retained, p[j_retained as usize], z);
            let z: i64 = iter + (cube(rand::thread_rng().gen::<f64>()) * tabu_duration as f64) as i64;
            set(&mut *tabu_list, j_retained, p[i_retained as usize], z);

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
            for i in 0..(n as i64 - 1) {
                for j in (i + 1)..(n as i64) {
                    if i != i_retained && i != j_retained &&
                        j != i_retained && j != j_retained {
                        let y: i64 = delta_part(&a, &b, &dist, &p, i, j, i_retained, j_retained);
                        set(&mut *dist, i, j, y);
                    } else {
                        let y: i64 = delta(&a, &b, &p, i, j);
                        set(&mut *dist, i, j, y);
                    }
                }
            }
        }
    }
    p = best_sol;
    best_cost
}

fn len(v: &Vector) -> i64 {
    v.len() as i64
}

fn new_matrix(n: i64) -> Box<Matrix> {
    Box::new(Matrix::new(n))
}

fn copy_vector(v: &mut Vector, w: &Vector) {
    for i in 0..w.len() {
        v[i] = w[i];
    }
}

fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m.a[(i * m.n + j) as usize]
}

fn delta(a: &Matrix, b: &Matrix, p: &Vector, r: i64, s: i64) -> i64 {
    let mut d: i64 = (get(a, r, r) - get(a, s, s)) * (get(b, p[s as usize], p[s as usize]) - get(b, p[r as usize], p[r as usize])) +
        (get(a, r, s) - get(a, s, r)) * (get(b, p[s as usize], p[r as usize]) - get(b, p[r as usize], p[s as usize]));
    for i in 0..len(p) {
        if i != r && i != s {
            d += (get(a, i, r) - get(a, i, s)) * (get(b, p[i as usize], p[s as usize]) - get(b, p[i as usize], p[r as usize])) +
                (get(a, r, i) - get(a, s, i)) * (get(b, p[s as usize], p[i as usize]) - get(b, p[r as usize], p[i as usize]));
        }
    }
    d
}

fn set(m: &mut Matrix, i: i64, j: i64, v: i64) {
    m.a[(i * m.n + j) as usize] = v;
}

fn swap(p: &mut Vector, i: i64, j: i64) {
    let x: i64 = p[i as usize];
    p[i as usize] = p[j as usize];
    p[j as usize] = x;
}

fn cube(x: f64) -> f64 {
    x * x * x
}

fn print_vector(v: &Vector) {
    for i in 0..v.len() {
        print!("{} ", v[i]);
    }
    println!();
}

fn delta_part(a: &Matrix, b: &Matrix, dist: &Matrix, p: &Vector, i: i64, j: i64, r: i64, s: i64) -> i64 {
    get(dist, i, j) + (get(a, r, i) - get(a, r, j) + get(a, s, j) - get(a, s, i)) *
        (get(b, p[s as usize], p[i as usize]) - get(b, p[s as usize], p[j as usize]) + get(b, p[r as usize], p[j as usize]) - get(b, p[r as usize], p[i as usize])) +
        (get(a, i, r) - get(a, j, r) + get(a, j, s) - get(a, i, s)) *
            (get(b, p[i as usize], p[s as usize]) - get(b, p[j as usize], p[s as usize]) + get(b, p[j as usize], p[r as usize]) - get(b, p[i as usize], p[r as usize]))
}

type Vector = Vec<i64>;

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn new(n: i64) -> Matrix {
        Matrix {
            n: n,
            a: vec![0; (n * n) as usize],
        }
    }
}

impl Index<usize> for Matrix {
    type Output = i64;

    fn index(&self, index: usize) -> &i64 {
        &self.a[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut i64 {
        &mut self.a[index]
    }
}

