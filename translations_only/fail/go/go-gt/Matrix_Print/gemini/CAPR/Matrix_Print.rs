
fn print_matrix(m: &Matrix) {
    for i in 0..m.n {
        for j in 0..m.n {
            print!("{} ", m.a[(i * m.n + j) as usize]);
        }
        println!();
    }
}

fn get_matrix(m: &Matrix, i: i64, j: i64) -> i64 {
    m.a[(i * m.n + j) as usize]
}

struct Matrix {
    n: i64,
    a: Vec<i64>,
}
