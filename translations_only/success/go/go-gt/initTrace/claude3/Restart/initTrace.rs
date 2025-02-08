
struct Matrix {
    n: i64,
    a: Box<[i64]>,
}

fn init_trace(n: i64, inc: i64, mut trace: Box<Matrix>) {
    let mut i = 0;
    while i < n {
        let mut j = 0;
        while j < n {
            set(&mut trace, i, j, inc);
            j += 1;
        }
        i += 1;
    }
}

fn set(m: &mut Matrix, i: i64, j: i64, v: i64) {
    let idx = (i * m.n + j) as usize;
    m.a[idx] = v;
}
