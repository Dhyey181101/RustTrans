
struct Matrix {
    n: i64,
    a: Vec<i64>,
}

fn new_matrix(n: i64) -> Matrix {
    Matrix {
        n,
        a: vec![0; n as usize * n as usize],
    }
}

fn index_mut(matrix: &mut Matrix, index: usize) -> &mut i64 {
    &mut matrix.a[index]
}

fn init_trace(n: i64, inc: i64, matrix: &mut Matrix) {
    let mut i = 0;
    let mut j = 0;
    while i < n {
        while j < n {
            set_matrix_element(matrix, i, j, inc);
            j += 1;
        }
        j = 0;
        i += 1;
    }
}

fn set_matrix_element(matrix: &mut Matrix, i: i64, j: i64, value: i64) {
    *index_mut(matrix, i as usize * matrix.n as usize + j as usize) = value;
}

