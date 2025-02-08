
use rand::Rng;
use rand::thread_rng;

pub struct Vector([i64; 1000]);

pub struct Matrix {
n: i64,
a: Box<[i64]>,
}

pub fn QAP_SolveTS(
a: &Matrix,
b: &Matrix,
p: &Vector,
opt: i64,
tabu_duration: i64,
) {
let mut rng = thread_rng();
// use `rng` here
}
