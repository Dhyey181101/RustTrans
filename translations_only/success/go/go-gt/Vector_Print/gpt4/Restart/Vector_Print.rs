
use std::fmt::Write;

struct Vector(Vec<i64>);

fn print(v: &Vector) {
    let mut output = String::new();
    for &item in &v.0 {
        write!(&mut output, "{} ", item).unwrap();
    }
    println!("{}", output);
}
