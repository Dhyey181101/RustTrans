

use std::cmp::max;

fn LCSEditDistance(str1: &str, str2: &str) -> isize {
    if str1.len() == 0 {
        return str2.len() as isize;
    } else if str2.len() == 0 {
        return str1.len() as isize;
    } else if str1 == str2 {
        return 0;
    }

    let lcs = LCS(str1, str2);
    return (str1.len() as isize) - (lcs as isize) + (str2.len() as isize) - (lcs as isize);
}

fn LCS(str1: &str, str2: &str) -> usize {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    if rune_str1.len() == 0 || rune_str2.len() == 0 {
        return 0;
    } else if rune_str1 == rune_str2 {
        return rune_str1.len();
    }

    let lcs_matrix = lcs_process(&rune_str1, &rune_str2);
    return lcs_matrix[rune_str1.len()][rune_str2.len()];
}

fn Equal(a: &[char], b: &[char]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }
    return true;
}

fn lcs_process(rune_str1: &Vec<char>, rune_str2: &Vec<char>) -> Vec<Vec<usize>> {
    let mut lcs_matrix: Vec<Vec<usize>> = vec![vec![0; rune_str2.len() + 1]; rune_str1.len() + 1];

    for i in 1..rune_str1.len() + 1 {
        for j in 1..rune_str2.len() + 1 {
            if rune_str1[i - 1] == rune_str2[j - 1] {
                lcs_matrix[i][j] = lcs_matrix[i - 1][j - 1] + 1;
            } else {
                lcs_matrix[i][j] = max(lcs_matrix[i][j - 1], lcs_matrix[i - 1][j]);
            }
        }
    }

    return lcs_matrix;
}

