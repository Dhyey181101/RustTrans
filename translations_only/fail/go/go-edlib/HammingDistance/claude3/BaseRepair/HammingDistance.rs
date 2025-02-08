
fn hamming_distance(str1: &str, str2: &str) -> Result<isize, &'static str> {
    let str1_chars: Vec<char> = str1.chars().collect();
    let str2_chars: Vec<char> = str2.chars().collect();

    if str1_chars.len() != str2_chars.len() {
        return Err("Undefined for strings of unequal length");
    } else if are_equal(&str1_chars, &str2_chars) {
        return Ok(0);
    }

    let mut counter: isize = 0;
    for i in 0..str1_chars.len() {
        if str1_chars[i] != str2_chars[i] {
            counter += 1;
        }
    }

    Ok(counter)
}

fn are_equal(a: &Vec<char>, b: &Vec<char>) -> bool {
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
