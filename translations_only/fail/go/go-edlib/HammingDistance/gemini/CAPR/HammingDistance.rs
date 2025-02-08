
use std::cmp::min;

fn hamming_distance(str1: &str, str2: &str) -> Result<isize, String> {
    if str1.len() != str2.len() {
        return Err(String::from("Undefined for strings of unequal length"));
    }
    let mut counter = 0;
    for (c1, c2) in str1.chars().zip(str2.chars()) {
        if c1 != c2 {
            counter += 1;
        }
    }
    Ok(counter)
}

fn equal(a: &str, b: &str) -> bool {
    a.len() == b.len() && a.chars().zip(b.chars()).all(|(c1, c2)| c1 == c2)
}
