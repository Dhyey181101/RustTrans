
type Vector = Vec<i64>;

fn copy(v: &mut Vector, w: &Vector) {
    for i in 0..v.len() {
        v[i] = w[i];
    }
}
