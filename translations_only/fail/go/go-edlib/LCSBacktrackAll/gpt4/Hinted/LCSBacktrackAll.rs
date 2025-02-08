
use std::collections::HashSet;
use std::error::Error;

fn lcs_backtrack_all(str1: &str, str2: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    if rune_str1.is_empty() || rune_str2.is_empty() {
        return Err("Can't process and backtrack any LCS with empty string".into());
    } else if rune_str1 == rune_str2 {
        return Ok(vec![str1.to_string()]);
    }

    let lcs_matrix = lcs_process(&rune_str1, &rune_str2);
    let substrings = process_lcs_backtrack_all(str1, str2, &lcs_matrix, rune_str1.len(), rune_str2.len());
    Ok(substrings.into_iter().collect())
}

fn lcs_process(rune_str1: &[char], rune_str2: &[char]) -> Vec<Vec<isize>> {
    let mut lcs_matrix = vec![vec![0; rune_str2.len() + 1]; rune_str1.len() + 1];

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

fn process_lcs_backtrack_all(str1: &str, str2: &str, lcs_matrix: &[Vec<isize>], m: usize, n: usize) -> HashSet<String> {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();
    let mut substrings = HashSet::new();

    if m == 0 || n == 0 {
        substrings.insert("".to_string());
    } else if rune_str1[m - 1] == rune_str2[n - 1] {
        for key in process_lcs_backtrack_all(str1, str2, lcs_matrix, m - 1, n - 1) {
            substrings.insert(format!("{}{}", key, rune_str1[m - 1]));
        }
    } else {
        if lcs_matrix[m - 1][n] >= lcs_matrix[m][n - 1] {
            substrings.extend(process_lcs_backtrack_all(str1, str2, lcs_matrix, m - 1, n));
        }
        if lcs_matrix[m][n - 1] >= lcs_matrix[m - 1][n] {
            substrings.extend(process_lcs_backtrack_all(str1, str2, lcs_matrix, m, n - 1));
        }
    }

    substrings
}
