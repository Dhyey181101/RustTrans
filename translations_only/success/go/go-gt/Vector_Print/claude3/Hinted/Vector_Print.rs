
use std::fmt;

type Vector = Vec<i64>;

fn print(v: Vector) {
    for i in v {
        print!("{} ", i);
    }
    println!();
}
