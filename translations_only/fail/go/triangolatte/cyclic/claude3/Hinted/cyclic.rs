
fn cyclic(i: i32, n: i32) -> i32 {
    if n == 0 {
        panic!("Input is invalid, crash gracefully");
    }
    let n = n.abs();
    (i % n + n) % n
}

fn main() {
    println!("{}", cyclic(16776960, 940504832)); // 16776960
    println!("{}", cyclic(148338434, 0)); // panics
    println!("{}", cyclic(8484858, -2)); // 8484858
    println!("{}", cyclic(16842752, -939523841)); // 16842752
}
