
use std::fmt;

fn HammingDistance(str1: &str, str2: &str) -> Result<isize, &'static str> {
    // Convert strings to slice of bytes to handle no-ASCII characters
    let rune_str1 = str1.as_bytes();
    let rune_str2 = str2.as_bytes();

    if rune_str1.len() != rune_str2.len() {
        return Err("Undefined for strings of unequal length");
    } else if Equal(rune_str1, rune_str2) {
        return Ok(0);
    }

    let mut counter = 0;
    for i in 0..rune_str1.len() {
        if rune_str1[i] != rune_str2[i] {
            counter += 1;
        }
    }

    Ok(counter)
}

fn Equal(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for (i, v) in a.iter().enumerate() {
        if v != &b[i] {
            return false;
        }
    }
    true
}

struct HammingError;

impl fmt::Display for HammingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Undefined for strings of unequal length")
    }
}

