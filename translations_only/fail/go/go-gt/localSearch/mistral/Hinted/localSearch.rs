

use rand::Rng;
use std::cmp::Ordering;
use rand::seq::SliceRandom; // added

type Vector = Vec<i64>;
type Matrix = StructMatrix;

struct StructMatrix {
    n: i64,
    a: Vec<i64>,
}

impl StructMatrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}

fn local_search(a: &Matrix, b: &Matrix, mut p: Vector, cost: &mut i64) {
    let mut rng = rand::thread_rng();
    let n = p.len() as i64;
    let mut move_set = create_move_set(n);
    let mut improved = true;
    for _ in 0..2 {
        if !improved {
            break;
        }
        move_set.shuffle(&mut rng);
        for i in 0..move_set.len() {
            let move_i = move_set[i];
            let r = move_i / n;
            let s = move_i % n;
            let d = delta(a, b, p.clone(), r, s);
            if d < 0 {
                *cost += d;
                let tmp = p[r as usize];
                p[r as usize] = p[s as usize];
                p[s as usize] = tmp;
                improved = true;
            }
        }
    }
}

fn create_move_set(n: i64) -> Vector {
    let mut move_set = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < n - 1 {
        j = i + 1;
        while j < n {
            move_set.push(n * i + j);
            j += 1;
        }
        i += 1;
    }
    move_set
}

fn delta(a: &Matrix, b: &Matrix, p: Vector, r: i64, s: i64) -> i64 {
    let mut d = 0;
    let n = p.len() as i64;
    d = (a.get(r, r) - a.get(s, s)) * (b.get(p[s as usize], p[s as usize]) - b.get(p[r as usize], p[r as usize]))
        + (a.get(r, s) - a.get(s, r)) * (b.get(p[s as usize], p[r as usize]) - b.get(p[r as usize], p[s as usize]));
    for i in 0..n {
        if i != r && i != s {
            d += (a.get(i, r) - a.get(i, s)) * (b.get(p[i as usize], p[s as usize]) - b.get(p[i as usize], p[r as usize]))
                + (a.get(r, i) - a.get(s, i)) * (b.get(p[s as usize], p[i as usize]) - b.get(p[r as usize], p[i as usize]));
        }
    }
    d
}

fn shuffle<R: Rng>(v: &mut Vector, rng: &mut R) {
    for i in (1..v.len()).rev() {
        let j = rng.gen_range(0..=i);
        if i != j {
            v.swap(i, j);
        }
    }
}

