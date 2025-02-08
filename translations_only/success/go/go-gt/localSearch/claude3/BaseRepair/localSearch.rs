
use rand::Rng;

type Vector = Vec<i64>;

fn local_search(a: Box<Matrix>, b: Box<Matrix>, mut p: Vector, cost: &mut i64) {
    let n = p.len() as i64;
    let mut move_vec = vec![0; (n * (n - 1) / 2) as usize];
    let mut n_mov = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            move_vec[n_mov as usize] = n * i + j;
            n_mov += 1;
        }
    }
    let mut improved = true;
    for _ in 0..2 {
        if !improved {
            break;
        }
        improved = false;
        for i in 0..n_mov - 1 {
            move_vec.swap(i as usize, unif((i + 1) as usize, (n_mov - 1) as usize));
        }
        for i in 0..n_mov {
            let r = move_vec[i as usize] / n;
            let s = move_vec[i as usize] % n;
            let d = delta(&a, &b, &p, r, s);
            if d < 0 {
                *cost += d;
                p.swap(r as usize, s as usize);
                improved = true;
            }
        }
    }
}

fn unif(low: usize, high: usize) -> usize {
    let mut rng = rand::thread_rng();
    low + rng.gen_range(0..=high - low)
}

fn delta(a: &Matrix, b: &Matrix, p: &Vector, r: i64, s: i64) -> i64 {
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

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}
