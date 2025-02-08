
extern crate rand;

use rand::Rng;
use std::boxed::Box;

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

fn new_env(n: i64) -> Box<Env> {
    let mut rng = rand::thread_rng();
    let mut e = Box::new(Env {
        m: 0,
        n: n,
        t: Vec::new(),
        s: Vec::new(),
        slack: Vec::new(),
        slackx: Vec::new(),
        prev: Vec::new(),
        xy: Vec::new(),
        yx: Vec::new(),
        lx: Vec::new(),
        ly: Vec::new(),
    });
    e.t = vec![false; n as usize];
    e.s = vec![false; n as usize];
    e.slack = vec![0; n as usize];
    e.slackx = vec![0; n as usize];
    e.prev = vec![-1; n as usize];
    e.xy = vec![-1; n as usize];
    e.yx = vec![-1; n as usize];
    e.lx = vec![rng.gen(); n as usize];
    e.ly = vec![rng.gen(); n as usize];
    e
}
