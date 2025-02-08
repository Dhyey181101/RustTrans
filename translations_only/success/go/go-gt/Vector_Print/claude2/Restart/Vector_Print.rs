
use std::fmt;

struct Vector(Box<[i64]>);

fn print(v: Vector) {
    for i in 0..v.0.len() {
        print!("{} ", v.0[i]);
    }
    println!("");
}
