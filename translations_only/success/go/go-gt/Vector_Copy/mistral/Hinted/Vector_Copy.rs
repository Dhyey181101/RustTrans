

use std::mem;
use libc::int64_t;

struct Vector(Box<[int64_t; 100]>);

fn copy(mut v: Vector, w: Vector) {
    for i in 0..100 {
        v.0[i] = w.0[i];
    }
}

