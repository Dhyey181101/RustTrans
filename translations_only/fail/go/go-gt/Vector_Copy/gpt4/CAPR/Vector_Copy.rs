
struct Vector(Vec<i64>);

fn copy(v: &mut Vector, w: &Vector) {
    for (i, item) in w.0.iter().enumerate() {
        if i < v.0.len() {
            v.0[i] = *item;
        }
    }
}

fn main() {}
