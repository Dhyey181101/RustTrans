
fn equal(a: Box<[char]>, b: Box<[char]>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for (i, &v) in a.iter().enumerate() {
        if v != b[i] {
            return false;
        }
    }
    true
}

fn main() {
    // Example 0
    let a: Box<[char]> = Box::new([]);
    let b: Box<[char]> = Box::new([]);
    println!("{}", equal(a, b)); // Expected Output: true

    // Example 1
    let a: Box<[char]> = Box::new(['᐀']);
    let b: Box<[char]> = Box::new(['\x00', '\x00']);
    println!("{}", equal(a, b)); // Expected Output: false

    // Example 2
    let a: Box<[char]> = Box::new(['\u{100800}']);
    let b: Box<[char]> = Box::new([]);
    println!("{}", equal(a, b)); // Expected Output: false
}
