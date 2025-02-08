
fn cyclic(i: i32, n: i32) -> i32 {
    if n == 0 {
        panic!("Input is invalid, crash gracefully");
    }
    (i % n + n) % n
}

fn main() {
    let input_1 = (117440523, 0);
    let input_2 = (16722465, 553648128);
    let input_3 = (-14013942, -10422272);
    let input_4 = (720875519, -1976762367);

    for (i, n) in [input_1, input_2, input_3, input_4].into_iter() {
        println!("cyclic({}, {}) = {}", i, n, cyclic(i, n));
    }
}
