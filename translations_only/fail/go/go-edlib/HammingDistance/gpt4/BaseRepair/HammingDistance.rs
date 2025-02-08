
fn hamming_distance(str1: &str, str2: &str) -> Result<isize, Box<dyn std::error::Error>> {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    if rune_str1.len() != rune_str2.len() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Undefined for strings of unequal length")));
    } else if equal(&rune_str1, &rune_str2) {
        return Ok(0);
    }

    let mut counter: isize = 0;
    for i in 0..rune_str1.len() {
        if rune_str1[i] != rune_str2[i] {
            counter += 1;
        }
    }

    Ok(counter)
}

fn equal(a: &[char], b: &[char]) -> bool {
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
