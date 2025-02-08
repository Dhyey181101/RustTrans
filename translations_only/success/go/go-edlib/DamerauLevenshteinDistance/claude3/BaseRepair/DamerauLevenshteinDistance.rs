
use std::cmp::min;

fn damerau_levenshtein_distance(str1: &str, str2: &str) -> isize {
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

    let mut da = std::collections::HashMap::new();
    for i in 0..runestr1len {
        da.insert(runestr1[i], 0);
    }
    for i in 0..runestr2len {
        da.insert(runestr2[i], 0);
    }

    let mut matrix = vec![vec![0; runestr2len + 2]; runestr1len + 2];

    let max_dist = (runestr1len + runestr2len) as isize;

    matrix[0][0] = max_dist;
    for i in 0..=runestr1len {
        matrix[i + 1][0] = max_dist;
        matrix[i + 1][1] = i as isize;
    }
    for i in 0..=runestr2len {
        matrix[0][i + 1] = max_dist;
        matrix[1][i + 1] = i as isize;
    }

    for i in 1..=runestr1len {
        let mut db = 0;
        for j in 1..=runestr2len {
            let i1 = *da.get(&runestr2[j - 1]).unwrap_or(&0);
            let j1 = db;
            let cost = if runestr1[i - 1] == runestr2[j - 1] {
                db = j;
                0
            } else {
                1
            };

            matrix[i + 1][j + 1] = min(
                min(
                    matrix[i + 1][j] + 1,
                    matrix[i][j + 1] + 1,
                ),
                min(
                    matrix[i][j] + cost,
                    matrix[i1][j1] + (i - i1 - 1) as isize + 1 + (j - j1 - 1) as isize,
                ),
            );
        }

        *da.get_mut(&runestr1[i - 1]).unwrap_or_else(|| panic!()) = i;
    }

    matrix[runestr1len + 1][runestr2len + 1]
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
