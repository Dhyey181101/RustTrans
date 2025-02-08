
fn update(e: &mut Env) {
    let mut i: i64 = 0;
    let mut d: i64 = i64::MAX;
    while i < e.N {
        if !e.T[i as usize] {
            d = d.min(e.Slack[i as usize]);
        }
        i += 1;
    }
    i = 0;
    while i < e.N {
        if e.S[i as usize] {
            e.Lx[i as usize] -= d;
        }
        i += 1;
    }
    i = 0;
    while i < e.N {
        if e.T[i as usize] {
            e.Ly[i as usize] += d;
        }
        i += 1;
    }
    i = 0;
    while i < e.N {
        if !e.T[i as usize] {
            e.Slack[i as usize] -= d;
        }
        i += 1;
    }
}

fn min(a: i64, b: i64) -> i64 {
    if a < b {
        a
    } else {
        b
    }
}

struct Env {
    N: i64,
    T: Vec<bool>,
    S: Vec<bool>,
    Slack: Vec<i64>,
    Lx: Vec<i64>,
    Ly: Vec<i64>,
}
