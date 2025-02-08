
fn levenshtein_distance(str1: &str, str2: &str) -> isize {
    let runestr1: Vec<char> = str1.chars().collect();
    let runestr2: Vec<char> = str2.chars().collect();

    let runestr1len = runestr1.len();
    let runestr2len = runestr2.len();

    if runestr1len == 0 {
        return runestr2len as isize;
    } else if runestr2len == 0 {
        return runestr1len as isize;
    } else if equal(&runestr1, &runestr2) {
        return 0;
    }

    let mut column: Vec<isize> = vec![0; runestr1len + 1];

    for y in 1..=runestr1len {
        column[y] = y as isize;
    }

    for x in 1..=runestr2len {
        column[0] = x as isize;
        let mut lastkey = (x - 1) as isize;
        for y in 1..=runestr1len {
            let oldkey = column[y];
            let i = if runestr1[y - 1] != runestr2[x - 1] {
                1
            } else {
                0
            };
            column[y] = min(
                min(column[y] + 1, column[y - 1] + 1),
                lastkey + i as isize,
            );
            lastkey = oldkey;
        }
    }

    column[runestr1len]
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

fn min(a: isize, b: isize) -> isize {
    if b < a {
        b
    } else {
        a
    }
}
