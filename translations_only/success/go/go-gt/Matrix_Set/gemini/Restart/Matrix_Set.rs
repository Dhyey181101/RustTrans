
pub fn set(m: &mut Matrix, i: i64, j: i64, v: i64) {
    m.a[(i * m.n + j) as usize] = v;
}

pub struct Matrix {
    pub n: i64,
    pub a: Vec<i64>,
}
