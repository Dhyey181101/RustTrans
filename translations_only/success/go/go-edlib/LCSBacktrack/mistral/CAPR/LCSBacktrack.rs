
use std::cmp::max;

pub fn lcs_backtrack(str1: &str, str2: &str) -> Result<String, String> {
    let rune_str1 = str1.chars().collect::<Vec<char>>();
    let rune_str2 = str2.chars().collect::<Vec<char>>();

    if rune_str1.is_empty() || rune_str2.is_empty() {
        return Err(String::from(
            "Can't process and backtrack any LCS with empty string",
        ));
    } else if are_equal(&rune_str1, &rune_str2) {
        return Ok(str1.to_string());
    }

    let lcs_matrix = lcs_process(&rune_str1, &rune_str2);
    Ok(process_lcs_backtrack(
        str1,
        str2,
        &lcs_matrix,
        rune_str1.len(),
        rune_str2.len(),
    ))
}

fn are_equal(a: &[char], b: &[char]) -> bool {
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

fn lcs_process(rune_str1: &[char], rune_str2: &[char]) -> Vec<Vec<i32>> {
    let mut lcs_matrix: Vec<Vec<i32>> = vec![vec![0; rune_str2.len() + 1]; rune_str1.len() + 1];

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

fn process_lcs_backtrack(
    str1: &str,
    str2: &str,
    lcs_matrix: &[Vec<i32>],
    m: usize,
    n: usize,
) -> String {
    if m == 0 || n == 0 {
        return String::new();
    } else if str1.chars().nth(m - 1).unwrap() == str2.chars().nth(n - 1).unwrap() {
        format!(
            "{}{}",
            process_lcs_backtrack(str1, str2, lcs_matrix, m - 1, n - 1),
            str1.chars().nth(m - 1).unwrap()
        )
    } else if lcs_matrix[m][n - 1] > lcs_matrix[m - 1][n] {
        process_lcs_backtrack(str1, str2, lcs_matrix, m, n - 1)
    } else {
        process_lcs_backtrack(str1, str2, lcs_matrix, m - 1, n)
    }
}
