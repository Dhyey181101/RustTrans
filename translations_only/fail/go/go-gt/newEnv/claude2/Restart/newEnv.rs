
use std::boxed::Box;

fn new_env(n: i64) -> Box<Env> {
    let mut e = Box::new(Env::new());
    e.m = 0;
    e.n = n;
    e.t = vec![false; n as usize];
    e.s = vec![false; n as usize];
    e.slack = vec![0; n as usize];
    e.slackx = vec![0; n as usize];
    e.prev = vec![0; n as usize];
    e.xy = vec![0; n as usize];
    e.yx = vec![0; n as usize];
    e.lx = vec![0; n as usize];
    e.ly = vec![0; n as usize];
    for i in 0..n {
        e.xy[i as usize] = -1;
        e.yx[i as usize] = -1;
    }
    e
}

struct Env {
    m: i64,
    n: i64,
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

impl Env {
    fn new() -> Self {
        Self {
            m: 0,
            n: 0,
            t: Vec::new(),
            s: Vec::new(),
            slack: Vec::new(),
            slackx: Vec::new(),
            prev: Vec::new(),
            xy: Vec::new(),
            yx: Vec::new(),
            lx: Vec::new(),
            ly: Vec::new(),
        }
    }
}
