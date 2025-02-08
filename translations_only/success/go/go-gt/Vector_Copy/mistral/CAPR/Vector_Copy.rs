

use std::mem;
use libc::int64_t;

struct Vector(Box<[int64_t; 1000]>);

fn copy(mut v: Vector, mut w: Vector) {
    for i in 0..1000 {
        v.0[i] = w.0[i];
    }
    mem::swap(&mut v.0, &mut w.0);
}

