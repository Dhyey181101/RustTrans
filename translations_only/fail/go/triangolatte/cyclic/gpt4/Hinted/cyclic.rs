
fn cyclic(i: i32, n: i32) -> i32 {
    if n == 0 {
        panic!("Input is invalid, crash gracefully");
    }
    (i % n + n) % n
}

fn main() {
    println!("{}", cyclic(167772288, 792640768)); // Example 0
    // println!("{}", cyclic(87947066, 0)); // Example 1, this will panic
    println!("{}", cyclic(-67109096, 118106890)); // Example 2
    println!("{}", cyclic(-1107948859, -67)); // Example 3
}
