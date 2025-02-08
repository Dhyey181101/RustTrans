
use std::cmp::max as std_max;

fn lcs(str1: &str, str2: &str) -> isize {
    let rune_str1 = str1.chars().collect::<Vec<_>>();
    let rune_str2 = str2.chars().collect::<Vec<_>>();

    if rune_str1.is_empty() || rune_str2.is_empty() {
        0
    } else if equal(rune_str1.clone(), rune_str2.clone()) {
        rune_str1.len() as isize
    } else {
        let lcs_matrix = lcs_process(&rune_str1, &rune_str2);
        lcs_matrix[rune_str1.len()][rune_str2.len()]
    }
}

fn equal(mut a: Vec<char>, b: Vec<char>) -> bool {
    if a.len() != b.len() {
        false
    } else {
        a.iter_mut()
            .zip(b)
            .all(|(x, y)| *x == y)
    }
}

fn lcs_process(rune_str1: &Vec<char>, rune_str2: &Vec<char>) -> Vec<Vec<isize>> {
    let mut lcs_matrix = vec![vec![0; rune_str2.len() + 1]; rune_str1.len() + 1];

    for i in 1..=rune_str1.len() {
        for j in 1..=rune_str2.len() {
            if rune_str1[i - 1] == rune_str2[j - 1] {
                lcs_matrix[i][j] = lcs_matrix[i - 1][j - 1] + 1;
            } else {
                lcs_matrix[i][j] = std_max(lcs_matrix[i][j - 1], lcs_matrix[i - 1][j]);
            }
        }
    }

    lcs_matrix
}

