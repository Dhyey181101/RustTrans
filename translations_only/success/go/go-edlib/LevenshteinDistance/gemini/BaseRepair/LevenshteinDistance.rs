
fn levenshtein_distance(str1: &str, str2: &str) -> isize {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    let rune_str1_len = rune_str1.len();
    let rune_str2_len = rune_str2.len();

    if rune_str1_len == 0 {
        return rune_str2_len as isize;
    } else if rune_str2_len == 0 {
        return rune_str1_len as isize;
    } else if rune_str1 == rune_str2 {
        return 0;
    }

    let mut column: Vec<isize> = vec![0; rune_str1_len + 1];

    for y in 1..=rune_str1_len {
        column[y] = y as isize;
    }

    for x in 1..=rune_str2_len {
        column[0] = x as isize;
        let mut last_key = x as isize - 1;

        for y in 1..=rune_str1_len {
            let old_key = column[y];
            let mut i = 0;

            if rune_str1[y - 1] != rune_str2[x - 1] {
                i = 1;
            }

            column[y] = min(min(column[y] + 1, column[y - 1] + 1), last_key + i);
            last_key = old_key;
        }
    }

    column[rune_str1_len]
}

fn min(a: isize, b: isize) -> isize {
    if b < a {
        b
    } else {
        a
    }
}
