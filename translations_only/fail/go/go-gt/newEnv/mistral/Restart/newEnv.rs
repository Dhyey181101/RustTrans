
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
    let m = rng.gen();
    let mut e = Box::new(Env {
        m,
        n,
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
    for i in 0..n {
        e.xy[i as usize] = -1;
        e.yx[i as usize] = -1;
    }
    e
}
