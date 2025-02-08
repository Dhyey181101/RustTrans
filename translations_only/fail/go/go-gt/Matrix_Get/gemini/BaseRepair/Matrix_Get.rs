
pub fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m.a[(i * m.n + j) as usize]
}

pub struct Matrix {
    pub n: i64,
    pub a: Vec<i64>,
}
