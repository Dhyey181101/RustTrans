
use std::cmp::max;

fn lcs(str1: &str, str2: &str) -> isize {
    let rune_str1 = str1.chars().collect::<Vec<char>>();
    let rune_str2 = str2.chars().collect::<Vec<char>>();

    if rune_str1.is_empty() || rune_str2.is_empty() {
        0
    } else if are_equal(&rune_str1, &rune_str2) {
        (rune_str1.len()) as isize
    } else {
        let lcs_matrix = lcs_process(&rune_str1, &rune_str2);
        lcs_matrix[rune_str1.len()][rune_str2.len()]
    }
}

fn are_equal(a: &[char], b: &[char]) -> bool {
    if a.len() != b.len() {
        false
    } else {
        a.iter().zip(b).all(|(&x, &y)| x == y)
    }
}

fn lcs_process(rune_str1: &[char], rune_str2: &[char]) -> Vec<Vec<isize>> {
    let mut lcs_matrix = vec![vec![0 as isize; rune_str2.len() + 1]; rune_str1.len() + 1];

    for i in 1..=rune_str1.len() {
        for j in 1..=rune_str2.len() {
            if rune_str1[i - 1] == rune_str2[j - 1] {
                lcs_matrix[i][j] = lcs_matrix[i - 1][j - 1] + 1;
            } else {
                lcs_matrix[i][j] = max(lcs_matrix[i][j - 1], lcs_matrix[i - 1][j]);
            }
        }
    }

    lcs_matrix
}
