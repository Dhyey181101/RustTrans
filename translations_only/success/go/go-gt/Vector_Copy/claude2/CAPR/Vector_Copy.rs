
use std::ops::DerefMut;

struct Vector(Box<[i64]>);

fn copy(v: &mut Vector, w: &Vector) {
    let v = v.0.deref_mut();
    let w = &*w.0;
    for i in 0..v.len() {
        v[i] = w[i];
    }
}
