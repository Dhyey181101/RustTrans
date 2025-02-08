
use std::boxed::Box;

struct Vector(Box<[i64]>);

fn len(v: Vector) -> i64 {
    v.0.len() as i64
}
