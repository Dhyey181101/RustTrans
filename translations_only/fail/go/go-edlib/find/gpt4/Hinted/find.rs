
fn find(slice: Vec<Vec<char>>, val: Vec<char>) -> isize {
    for (i, item) in slice.iter().enumerate() {
        if equal(item, &val) {
            return i as isize;
        }
    }
    -1
}

fn equal(a: &Vec<char>, b: &Vec<char>) -> bool {
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
    // Example usage
    let slice: Vec<Vec<char>> = vec![
        vec!['a', 'b', 'c'],
        vec!['d', 'e', 'f'],
    ];
    let val: Vec<char> = vec!['d', 'e', 'f'];
    println!("{}", find(slice, val)); // Expected output: 1
}
