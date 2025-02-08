
use std::cmp::Ordering;

const INF: i64 = i64::MAX;

struct Vector(Vec<i64>);

impl Vector {
    fn len(&self) -> i64 {
        self.0.len() as i64
    }
    
    fn copy(&mut self, other: &Vector) {
        self.0.copy_from_slice(&other.0);
    }
    
    fn swap(&mut self, i: i64, j: i64) {
        self.0.swap(i as usize, j as usize);
    }
    
    fn print(&self) {
        println!("{:?}", &self.0);
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

    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }

    fn set(&mut self, i: i64, j: i64, value: i64) {
        self.a[(i * self.n + j) as usize] = value;
    }
}

fn qap_solve_ts(
    a: &Matrix,
    b: &Matrix,
    p: &mut Vector,
    opt: i64,
    tabu_duration: i64,
    aspiration: i64,
    nr_iterations: i64,
) -> i64 {
    let mut best_cost = INF;
    let n = p.len();
    
    let mut dist = Matrix::new(n);
    let mut tabu_list = Matrix::new(n);
    let mut best_sol = Vector(Vec::with_capacity(n as usize));
    best_sol.copy(p);
    
    let mut current_cost = 0;
    for i in 0..n {
        for j in 0..n {
            current_cost += a.get(i, j) * b.get(p.0[i as usize], p.0[j as usize]);
            if i < j {
                dist.set(i, j, delta(a, b, p, i, j));
            }
        }
    }

    best_cost = current_cost;

    // Tabu list initialization
    for i in 0..n {
        for j in 0..n {
            tabu_list.set(i, j, -(n * i + j));
        }
    }

    let mut iter = 0;
    while iter < nr_iterations && best_cost > opt {
        // Find best move
        let mut i_retained = INF;
        let mut j_retained = INF;
        let mut min_dist = INF;
        let mut already_aspired = false;

        for i in 0..(n - 1) {
            for j in (i + 1)..n {
                let autorized = tabu_list.get(i, p.0[j as usize]) < iter
                    || tabu_list.get(j, p.0[i as usize]) < iter;

                let aspired = tabu_list.get(i, p.0[j as usize]) < iter - aspiration
                    || tabu_list.get(j, p.0[i as usize]) < iter - aspiration
                    || current_cost + dist.get(i, j) < best_cost;

                let new_dist = dist.get(i, j);
                if (aspired && !already_aspired)
                    || (aspired 
                        && already_aspired
                        && new_dist < min_dist)
                    || (!aspired
                        && !already_aspired
                        && new_dist < min_dist
                        && autorized)
                {
                    i_retained = i;
                    j_retained = j;
                    min_dist = new_dist;
                    if aspired {
                        already_aspired = true;
                    }
                }
            }
        }

        if i_retained == INF {
            println!("All moves are tabu!");
        } else {
            p.swap(i_retained, j_retained);

            // Update solution value
            current_cost += dist.get(i_retained, j_retained);

            // Forbid reverse move
            let z = iter + rand::random::<f64>().powi(3) as i64 * tabu_duration;
            tabu_list.set(i_retained, p.0[j_retained as usize], z);
            let z = iter + rand::random::<f64>().powi(3) as i64 * tabu_duration;
            tabu_list.set(j_retained, p.0[i_retained as usize], z);

            // Best solution improved?
            match current_cost.cmp(&best_cost) {
                Ordering::Less => {
                    best_cost = current_cost;
                    best_sol.copy(p);
                    println!("Iteration {}: cost={}", iter, best_cost);
                    best_sol.print();
                }
                _ => {}
            }

            // Update move cost matrix
            for i in 0..(n - 1) {
                for j in (i + 1)..n {
                    if i != i_retained
                        && i != j_retained
                        && j != i_retained
                        && j != j_retained
                    {
                        let y = delta_part(
                            a,
                            b,
                            &dist,
                            p,
                            i,
                            j,
                            i_retained,
                            j_retained,
                        );
                        dist.set(i, j, y);
                    } else {
                        let y = delta(a, b, p, i, j);
                        dist.set(i, j, y);
                    }
                }
            }
        }

        iter += 1;
    }

    p.copy(&best_sol);
    best_cost
}

fn delta(
    a: &Matrix,
    b: &Matrix, 
    p: &Vector,
    r: i64,
    s: i64,
) -> i64 {
    let n = p.len();
    let mut sum = 0;

    for i in 0..n {
        if i != r && i != s {
            sum += (a.get(i, r) - a.get(i, s)) * (b.get(p.0[i as usize], p.0[s as usize]) - b.get(p.0[i as usize], p.0[r as usize]))
                + (a.get(r, i) - a.get(s, i)) * (b.get(p.0[s as usize], p.0[i as usize]) - b.get(p.0[r as usize], p.0[i as usize]));
        }
    }

    sum + (a.get(r, r) - a.get(s, s)) * (b.get(p.0[s as usize], p.0[s as usize]) - b.get(p.0[r as usize], p.0[r as usize]))
        + (a.get(r, s) - a.get(s, r)) * (b.get(p.0[s as usize], p.0[r as usize]) - b.get(p.0[r as usize], p.0[s as usize]))
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
    dist.get(i, j) + 
        (a.get(r, i) - a.get(r, j) + a.get(s, j) - a.get(s, i)) *
            (b.get(p.0[s as usize], p.0[i as usize]) - 
                b.get(p.0[s as usize], p.0[j as usize]) + 
                b.get(p.0[r as usize], p.0[j as usize]) -
                b.get(p.0[r as usize], p.0[i as usize])) +
        (a.get(i, r) - a.get(j, r) + a.get(j, s) - a.get(i, s)) * 
            (b.get(p.0[i as usize], p.0[s as usize]) -
                b.get(p.0[j as usize], p.0[s as usize]) +
                b.get(p.0[j as usize], p.0[r as usize]) - 
                b.get(p.0[i as usize], p.0[r as usize]))
}
