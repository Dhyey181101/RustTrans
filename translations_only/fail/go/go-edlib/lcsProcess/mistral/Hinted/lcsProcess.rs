

use std::cmp::max;

pub fn lcs_process(rune_str1: Vec<u32>, rune_str2: Vec<u32>) -> Vec<Vec<i32>> {
    let mut lcs_matrix: Vec<Vec<i32>> = vec![vec![0; rune_str2.len() + 1]; rune_str1.len() + 1];

    for i in 1..=rune_str1.len() {
        for j in 1..=rune_str2.len() {
            if rune_str1[i - 1] == rune_str2[j - 1] {
                lcs_matrix[i][j] = lcs_matrix[i - 1][j - 1] + 1;
            } else {
                lcs_matrix[i][j] = std::cmp::max(lcs_matrix[i][j - 1], lcs_matrix[i - 1][j]);
            }
        }
    }

    lcs_matrix
}

