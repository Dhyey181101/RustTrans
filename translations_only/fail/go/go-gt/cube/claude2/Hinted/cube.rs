
fn cube(x: f64) -> f64 {
    x * x * x
}

fn main() {
    let x = 7.767132034505672e-299;
    println!("{}", cube(x));

    let x = 2.052268775268671e-289; 
    println!("{}", cube(x));

    let x = -8.446859224902765e+298;
    println!("{}", cube(x));

    let x = -5.82767148787421e+303;
    println!("{}", cube(x));
}
