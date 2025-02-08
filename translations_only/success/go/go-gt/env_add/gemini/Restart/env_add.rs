
pub fn add(env: &mut Env, i: i64, p: i64) {
    env.s[i as usize] = true;
    env.prev[i as usize] = p;
    for j in 0..env.n {
        if env.lx[i as usize] + env.ly[i as usize] - get(&env.g, i, j) < env.slack[i as usize] {
            env.slack[i as usize] = env.lx[i as usize] + env.ly[i as usize] - get(&env.g, i, j);
            env.slackx[i as usize] = j;
        }
    }
}

pub fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m.a[(i * m.n + j) as usize]
}

pub struct Env {
    pub n: i64,
    pub g: Box<Matrix>,
    pub s: Vec<bool>,
    pub slack: Vec<i64>,
    pub slackx: Vec<i64>,
    pub prev: Vec<i64>,
    pub lx: Vec<i64>,
    pub ly: Vec<i64>,
}

pub struct Matrix {
    pub n: i64,
    pub a: Vec<i64>,
}
