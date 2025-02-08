
fn levenshtein_distance(str1: &str, str2: &str) -> isize {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    let rune_str1len = rune_str1.len() as isize;
    let rune_str2len = rune_str2.len() as isize;
    if rune_str1len == 0 {
        return rune_str2len;
    } else if rune_str2len == 0 {
        return rune_str1len;
    } else if equal(&rune_str1, &rune_str2) {
        return 0;
    }

    let mut column: Vec<isize> = vec![0; (rune_str1len + 1) as usize];
    for y in 1..=rune_str1len {
        column[y as usize] = y;
    }
    for x in 1..=rune_str2len {
        column[0] = x;
        let mut lastkey = x - 1;
        for y in 1..=rune_str1len {
            let oldkey = column[y as usize];
            let mut i = 0;
            if rune_str1[(y - 1) as usize] != rune_str2[(x - 1) as usize] {
                i = 1;
            }
            column[y as usize] = min(
                min(column[y as usize] + 1, column[(y - 1) as usize] + 1),
                lastkey + i,
            );
            lastkey = oldkey;
        }
    }

    column[rune_str1len as usize]
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

fn min(a: isize, b: isize) -> isize {
    if b < a {
        b
    } else {
        a
    }
}
