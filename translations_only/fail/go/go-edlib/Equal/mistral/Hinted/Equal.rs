
fn Equal(a: Vec<u32>, b: Vec<u32>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for (i, v) in a.into_iter().enumerate() {
        if v != b[i] {
            return false;
        }
    }
    true
}

fn main() {
    let a = vec![];
    let b = vec![];
    println!("{}", Equal(a, b));

    let a = vec![0x8000];
    let b = vec![0xa000];
    // This will panic with a message "thread 'main' panicked at 'index out of bounds: the len is 1 but the index is 0x8000', src/main.rs:11:24'
    Equal(a, b);

    let a = vec![0, 0x100, 0];
    let b = vec![];
    // This will panic with a message 'index out of bounds: the len is 0 but the index is 2'
    Equal(a, b);
}
