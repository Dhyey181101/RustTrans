
use std::cmp::min;

pub fn OSADamerauLevenshteinDistance(str1: String, str2: String) -> isize {
    let rune_str1 = str1.chars().collect::<Vec<char>>();
    let rune_str2 = str2.chars().collect::<Vec<char>>();

    let rune_str1len = rune_str1.len();
    let rune_str2len = rune_str2.len();

    if rune_str1len == 0 {
        return rune_str2len as isize;
    } else if rune_str2len == 0 {
        return rune_str1len as isize;
    } else if rune_str1 == rune_str2 {
        return 0;
    } else if rune_str1len < rune_str2len {
        return OSADamerauLevenshteinDistance(str2, str1);
    }

    let mut matrix = vec![vec![0; rune_str2len + 1]; 3];

    for i in 0..=rune_str2len {
        matrix[0][i] = i as isize;
    }

    let mut count: isize;
    for i in 1..=rune_str1len {
        matrix[i % 3][0] = i as isize;
        for j in 1..=rune_str2len {
            if rune_str1[i - 1] == rune_str2[j - 1] {
                count = 0;
            } else {
                count = 1;
            }

            matrix[i % 3][j] = min(min(matrix[(i - 1) % 3][j] + 1, matrix[i % 3][j - 1] + 1), matrix[(i - 1) % 3][j - 1] + count);

            if i > 1 && j > 1 && rune_str1[i - 1] == rune_str2[j - 2] && rune_str1[i - 2] == rune_str2[j - 1] {
                matrix[i % 3][j] = min(matrix[i % 3][j], matrix[(i - 2) % 3][j - 2] + 1);
            }
        }
    }

    matrix[rune_str1len % 3][rune_str2len]
}

pub fn Equal(a: Vec<char>, b: Vec<char>) -> bool {
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

pub fn Min(a: isize, b: isize) -> isize {
    if b < a {
        return b;
    }

    a
}
