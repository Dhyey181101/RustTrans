
struct Vector(Vec<i64>);

fn copy(v: &mut Vector, w: &Vector) {
    for i in 0..v.0.len() {
        v.0[i] = w.0[i];
    }
}

fn main() {}
