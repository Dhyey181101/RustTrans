
use std::cmp::min;

fn hamming_distance(str1: &str, str2: &str) -> Result<isize, String> {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    if rune_str1.len() != rune_str2.len() {
        return Err("Undefined for strings of unequal length".to_string());
    } else if rune_str1 == rune_str2 {
        return Ok(0);
    }

    let mut counter: isize = 0;
    for i in 0..min(rune_str1.len(), rune_str2.len()) {
        if rune_str1[i] != rune_str2[i] {
            counter += 1;
        }
    }

    Ok(counter)
}

fn equal(a: &Vec<char>, b: &Vec<char>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }
    true
}
