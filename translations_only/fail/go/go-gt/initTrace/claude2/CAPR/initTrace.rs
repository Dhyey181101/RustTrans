

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

fn set(matrix: &mut Matrix, i: i64, j: i64, v: i64) {
    matrix.a[i as usize * matrix.n as usize + j as usize] = v;
}

fn init_trace(n: i64, inc: i64, trace: &mut Matrix) {
    let mut i = 0;
    let mut j = 0;
    while i < n {
        while j < n {
            set(trace, i, j, inc);
            j += 1;
        }
        i += 1;
    }
}

