
use std::cmp::min;

fn osa_damerau_levenshtein_distance(str1: &str, str2: &str) -> isize {
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
    } else if runestr1len < runestr2len {
        return osa_damerau_levenshtein_distance(str2, str1);
    }

    let row = min(runestr1len + 1, 3);
    let mut matrix: Vec<Vec<isize>> = vec![vec![0; runestr2len + 1]; row];

    for i in 0..row {
        matrix[i][0] = i as isize;
    }

    for j in 0..=runestr2len {
        matrix[0][j] = j as isize;
    }

    for i in 1..=runestr1len {
        matrix[i % 3][0] = i as isize;
        for j in 1..=runestr2len {
            let mut count = 0;
            if runestr1[i - 1] != runestr2[j - 1] {
                count = 1;
            }

            matrix[i % 3][j] = min(
                min(matrix[(i - 1) % 3][j] + 1, matrix[i % 3][j - 1] + 1),
                matrix[(i - 1) % 3][j - 1] + count,
            );

            if i > 1 && j > 1 && runestr1[i - 1] == runestr2[j - 2] && runestr1[i - 2] == runestr2[j - 1] {
                matrix[i % 3][j] = min(matrix[i % 3][j], matrix[(i - 2) % 3][j - 2] + 1);
            }
        }
    }

    matrix[runestr1len % 3][runestr2len]
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
