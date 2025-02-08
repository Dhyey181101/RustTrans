
struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn new(n: i64, a: Vec<i64>) -> Self {
        Matrix { n, a }
    }

    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}

fn print(matrix: &Matrix) {
    for i in 0..matrix.n {
        for j in 0..matrix.n {
            print!("{} ", matrix.get(i, j));
        }
        println!();
    }
}

fn main() {
    let n = 3;
    let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let matrix = Matrix::new(n, a);
    print(&matrix);
}
