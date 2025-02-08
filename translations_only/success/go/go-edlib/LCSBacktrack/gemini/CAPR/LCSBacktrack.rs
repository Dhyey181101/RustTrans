
use std::collections::VecDeque;

fn lcs_backtrack(str1: &str, str2: &str) -> Result<String, Box<dyn std::error::Error>> {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    if rune_str1.is_empty() || rune_str2.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Can't process and backtrack any LCS with empty string",
        )));
    } else if rune_str1 == rune_str2 {
        return Ok(str1.to_string());
    }

    let lcs_matrix = lcs_process(&rune_str1, &rune_str2);
    let mut lcs = VecDeque::new();
    let mut i = rune_str1.len();
    let mut j = rune_str2.len();

    while i > 0 && j > 0 {
        if rune_str1[i - 1] == rune_str2[j - 1] {
            lcs.push_front(rune_str1[i - 1]);
            i -= 1;
            j -= 1;
        } else if lcs_matrix[i][j - 1] > lcs_matrix[i - 1][j] {
            j -= 1;
        } else {
            i -= 1;
        }
    }

    Ok(lcs.iter().collect())
}

fn lcs_process(rune_str1: &[char], rune_str2: &[char]) -> Vec<Vec<isize>> {
    let mut lcs_matrix: Vec<Vec<isize>> = Vec::new();

    for _ in 0..=rune_str1.len() {
        lcs_matrix.push(vec![0; rune_str2.len() + 1]);
    }

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

fn max(a: isize, b: isize) -> isize {
    if b > a {
        b
    } else {
        a
    }
}
