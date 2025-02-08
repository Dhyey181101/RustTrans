
use std::ops::DerefMut;

type Vector = Box<[i64]>;

fn copy(v: &mut Vector, w: &Vector) {
    let len = v.len();
    for i in 0..len {
        v[i] = w[i];
    }
}
