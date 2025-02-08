
pub fn new_env(n: i64) -> Box<Env> {
    let mut e = Box::new(Env {
        M: 0,
        N: n,
        T: vec![false; n as usize],
        S: vec![false; n as usize],
        Slack: vec![0; n as usize],
        Slackx: vec![0; n as usize],
        Prev: vec![0; n as usize],
        Xy: vec![-1; n as usize],
        Yx: vec![-1; n as usize],
        Lx: vec![0; n as usize],
        Ly: vec![0; n as usize],
    });
    for i in 0..n {
        e.Xy[i as usize] = -1;
        e.Yx[i as usize] = -1;
    }
    e
}

pub struct Env {
    pub M: i64,
    pub N: i64,

    pub T: Vec<bool>,
    pub S: Vec<bool>,
    pub Slack: Vec<i64>,
    pub Slackx: Vec<i64>,
    pub Prev: Vec<i64>,
    pub Xy: Vec<i64>,
    pub Yx: Vec<i64>,
    pub Lx: Vec<i64>,
    pub Ly: Vec<i64>,
}
