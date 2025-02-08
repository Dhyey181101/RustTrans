
fn main() {}

struct Matrix {
    n: i64,
    a: Box<[i64]>,
}

fn set(matrix: &mut Matrix, i: i64, j: i64, v: i64) {
    if let Some(index) = (i.checked_mul(matrix.n)).and_then(|x| x.checked_add(j)) {
        if index >= 0 && (index as usize) < matrix.a.len() {
            matrix.a[index as usize] = v;
        } else {
            panic!("Input is invalid, crash gracefully");
        }
    } else {
        panic!("Input is invalid, crash gracefully");
    }
}
