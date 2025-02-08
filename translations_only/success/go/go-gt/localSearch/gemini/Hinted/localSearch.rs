
use rand::Rng;

fn local_search(a: &Matrix, b: &Matrix, p: &mut Vec<i64>, cost: &mut i64) {
    // set of moves, numbered from 0 to index
    let mut i: i64;
    let mut j: i64;
    let mut n_mov: i64 = 0;
    let n = p.len() as i64;
    let mut move_ = vec![0; (n * (n - 1) / 2) as usize];
    n_mov = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            move_[n_mov as usize] = n * i + j;
            n_mov += 1;
        }
    }
    let mut improved = true;
    for k in 0..2 {
        improved = false;
        for i in 0..n_mov - 1 {
            move_.swap(i as usize, unif(i + 1, n_mov - 1) as usize);
        }
        for i in 0..n_mov {
            let r = move_[i as usize] / n;
            let s = move_[i as usize] % n;
            let d = delta(a, b, p, r, s);
            if d < 0 {
                *cost += d;
                p.swap(r as usize, s as usize);
                improved = true;
            }
        }
        if !improved {
            break;
        }
    }
}

fn unif(low: i64, high: i64) -> i64 {
    let mut rng = rand::thread_rng();
    low + rng.gen_range(0..high - low + 1)
}

fn delta(a: &Matrix, b: &Matrix, p: &Vec<i64>, r: i64, s: i64) -> i64 {
    let mut i: i64;
    let mut d = (a.get(r, r) - a.get(s, s)) * (b.get(p[s as usize], p[s as usize]) - b.get(p[r as usize], p[r as usize]))
        + (a.get(r, s) - a.get(s, r)) * (b.get(p[s as usize], p[r as usize]) - b.get(p[r as usize], p[s as usize]));
    for i in 0..p.len() as i64 {
        if i != r && i != s {
            d += (a.get(i, r) - a.get(i, s)) * (b.get(p[i as usize], p[s as usize]) - b.get(p[i as usize], p[r as usize]))
                + (a.get(r, i) - a.get(s, i)) * (b.get(p[s as usize], p[i as usize]) - b.get(p[r as usize], p[i as usize]));
        }
    }
    d
}

#[derive(Debug)]
struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}
