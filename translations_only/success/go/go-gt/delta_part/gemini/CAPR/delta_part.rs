
use std::collections::VecDeque;

fn delta_part(a: &Vec<Vec<i64>>, b: &Vec<Vec<i64>>, dist: &Vec<Vec<i64>>, p: &Vec<i64>, i: i64, j: i64, r: i64, s: i64) -> i64 {
    return (dist[i as usize][j as usize] + (a[r as usize][i as usize] - a[r as usize][j as usize] + a[s as usize][j as usize] - a[s as usize][i as usize]) *
        (b[p[s as usize] as usize][p[i as usize] as usize] - b[p[s as usize] as usize][p[j as usize] as usize] + b[p[r as usize] as usize][p[j as usize] as usize] - b[p[r as usize] as usize][p[i as usize] as usize]) +
        (a[i as usize][r as usize] - a[j as usize][r as usize] + a[j as usize][s as usize] - a[i as usize][s as usize]) *
            (b[p[i as usize] as usize][p[s as usize] as usize] - b[p[j as usize] as usize][p[s as usize] as usize] + b[p[j as usize] as usize][p[r as usize] as usize] - b[p[i as usize] as usize][p[r as usize] as usize]))
}

fn get(m: &Vec<Vec<i64>>, i: i64, j: i64) -> i64 {
    return m[i as usize][j as usize];
}

type Vector = Vec<i64>;

type Matrix = Vec<Vec<i64>>;
