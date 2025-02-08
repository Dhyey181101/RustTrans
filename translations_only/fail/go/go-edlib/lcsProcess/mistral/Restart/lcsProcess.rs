

use std::cmp::max;

pub fn lcs_process(rune_str1: &[char], rune_str2: &[char]) -> Vec<Vec<i32>> {
    let len_rune_str1 = rune_str1.len();
    let len_rune_str2 = rune_str2.len();

    let mut lcs_matrix = vec![vec![0; len_rune_str2 + 1]; len_rune_str1 + 1];

    for i in 1..=len_rune_str1 {
        for j in 1..=len_rune_str2 {
            if rune_str1[i - 1] == rune_str2[j - 1] {
                lcs_matrix[i][j] = lcs_matrix[i - 1][j - 1] + 1;
            } else {
                lcs_matrix[i][j] = std::cmp::max(lcs_matrix[i][j - 1], lcs_matrix[i - 1][j]);
            }
        }
    }

    lcs_matrix
}

