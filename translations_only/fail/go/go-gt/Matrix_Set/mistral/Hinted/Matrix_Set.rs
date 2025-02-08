
use std::mem;

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn set(&mut self, i: i64, j: i64, v: i64) {
        if i < 0 || j < 0 || i as usize >= self.a.len() / (self.n as usize) || j as usize >= self.n as usize {
            panic!("Input is invalid, crash gracefully");
        }
        self.a[i as usize * (self.n as usize) + j as usize] = v;
    }
}

fn main() {
    let mut matrix = Matrix {
        n: -4107845810132073718,
        a: vec![
            524416, 4881057858902294528, 163242311679, 34742514447155200, 3906368507716252962, -9223372036854775808,
            0, -9223372036854775808, -9223372036854775808, -9223372036854775808, -9223372036854775808,
            -9223372036854775808,
        ],
    };
    matrix.set(-9223372036854775808, 0, -9223372036854775808);
}
