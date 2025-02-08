
struct Vector(Vec<i64>);

fn swap(p: &mut Vector, i: usize, j: usize) {
    let x = p.0[i];
    p.0[i] = p.0[j];
    p.0[j] = x;
}
