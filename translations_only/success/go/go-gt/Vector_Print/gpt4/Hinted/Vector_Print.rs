
use std::fmt::Write;

struct Vector(Vec<i64>);

fn print(v: &Vector) {
    let mut output = String::new();
    for i in &v.0 {
        write!(&mut output, "{} ", i).unwrap();
    }
    println!("{}", output);
}
