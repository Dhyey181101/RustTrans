
struct Matrix {
    n: i64,
    a: Vec<i64>,
}

struct Env {
    m: i64,
    n: i64,
    g: Box<Matrix>,
    t: Vec<bool>,
    s: Vec<bool>,
    slack: Vec<i64>,
    slackx: Vec<i64>,
    prev: Vec<i64>,
    xy: Vec<i64>,
    yx: Vec<i64>,
    lx: Vec<i64>,
    ly: Vec<i64>,
}

fn init_h(g: Box<Matrix>) -> Box<Env> {
    let mut e = new_env(g.n);
    e.g = g;
    e.n = e.g.n;
    for i in 0..e.n {
        for j in 0..e.n {
            e.lx[i as usize] = max(e.lx[i as usize], e.g.get(i, j));
        }
    }
    e
}

fn new_env(n: i64) -> Box<Env> {
    let e = Box::new(Env {
        m: 0,
        n,
        g: Box::new(Matrix { n: 0, a: vec![] }),
        t: vec![false; n as usize],
        s: vec![false; n as usize],
        slack: vec![0; n as usize],
        slackx: vec![0; n as usize],
        prev: vec![0; n as usize],
        xy: vec![-1; n as usize],
        yx: vec![-1; n as usize],
        lx: vec![0; n as usize],
        ly: vec![0; n as usize],
    });
    e
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}

fn max(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}
