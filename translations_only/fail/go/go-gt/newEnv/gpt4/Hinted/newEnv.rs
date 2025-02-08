
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
    let mut env = Box::new(Env {
        m: 0,
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

    env
}
