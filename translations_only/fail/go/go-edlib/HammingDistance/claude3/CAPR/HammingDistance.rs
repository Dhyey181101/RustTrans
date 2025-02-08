
fn hamming_distance(str1: &str, str2: &str) -> Result<isize, &'static str> {
    let vec1: Vec<char> = str1.chars().collect();
    let vec2: Vec<char> = str2.chars().collect();

    if vec1.len() != vec2.len() {
        return Err("Undefined for strings of unequal length");
    } else if are_equal(&vec1, &vec2) {
        return Ok(0);
    }

    let mut counter: isize = 0;
    for i in 0..vec1.len() {
        if vec1[i] != vec2[i] {
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
