
use std::cmp::min;

fn levenshtein_distance(str1: &str, str2: &str) -> isize {
    let rune_str1 = str1.chars().collect::<Vec<char>>();
    let rune_str2 = str2.chars().collect::<Vec<char>>();

    let rune_str1len = rune_str1.len();
    let rune_str2len = rune_str2.len();

    if rune_str1len == 0 {
        return rune_str2len as isize;
    } else if rune_str2len == 0 {
        return rune_str1len as isize;
    } else if equal(&rune_str1, &rune_str2) {
        return 0;
    }

    let mut column = vec![0; rune_str1len + 1];

    for y in 1..=rune_str1len {
        column[y] = y as isize;
    }

    for x in 1..=rune_str2len {
        column[0] = x as isize;
        let lastkey = (x - 1) as usize;

        for y in 1..=rune_str1len {
            let oldkey = column[y];
            let i = if rune_str1[y - 1] != rune_str2[x - 1] { 1 } else { 0 };
            column[y] = min(
                min(column[y] + 1, column[y - 1] + 1),
                lastkey as isize + i as isize,
            );
            lastkey;
        }
    }

    column[rune_str1len]
}

fn equal(a: &[char], b: &[char]) -> bool {
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
