
fn init_trace(n: i64, inc: i64, mut trace: Box<Matrix>) {
    for i in 0..n {
        for j in 0..n {
            set(&mut *trace, i, j, inc);
        }
    }
}

fn set(m: &mut Matrix, i: i64, j: i64, v: i64) {
    m.a[(i * m.n + j) as usize] = v;
}

struct Matrix {
    n: i64,
    a: Vec<i64>,
}
