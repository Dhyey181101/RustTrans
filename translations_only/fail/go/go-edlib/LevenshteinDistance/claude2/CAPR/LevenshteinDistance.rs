
use std::cmp::Ordering;

fn levenshtein_distance(str1: &str, str2: &str) -> isize {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    let rune_str1len = rune_str1.len();
    let rune_str2len = rune_str2.len();
    if rune_str1len == 0 {
        return rune_str2len as isize;
    } else if rune_str2len == 0 {
        return rune_str1len as isize;
    } else if equal(&rune_str1, &rune_str2) {
        return 0;
    }

    let mut column: Vec<isize> = Vec::with_capacity(rune_str1len + 1);

    for y in 1..=rune_str1len {
        column.push(y as isize);
    }
    for x in 1..=rune_str2len {
        column[0] = x as isize;
        let lastkey = x - 1;
        for y in 1..=rune_str1len {
            let oldkey = column[y as usize];
            let i = if rune_str1[y - 1] != rune_str2[x - 1] {
                1
            } else {
                0
            };
            column[y as usize] = min(
                min(
                    column[y as usize] + 1, // insert
                    column[(y - 1) as usize] + 1, // delete
                ),
                lastkey as isize + i, // substitution
            );
            let _ = oldkey;
        }
    }

    column[rune_str1len as usize]
}

fn equal(a: &[char], b: &[char]) -> bool {
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

fn min(a: isize, b: isize) -> isize {
    match a.cmp(&b) {
        Ordering::Less => a,
        _ => b,
    }
}
