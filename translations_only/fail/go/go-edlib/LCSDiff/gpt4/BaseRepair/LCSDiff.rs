
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
struct LCSError;

impl fmt::Display for LCSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Can't process LCS diff with empty string")
    }
}

impl Error for LCSError {}

fn lcs_diff(str1: &str, str2: &str) -> Result<(Vec<String>, Vec<String>), LCSError> {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    if rune_str1.is_empty() || rune_str2.is_empty() {
        Err(LCSError)
    } else if equal(&rune_str1, &rune_str2) {
        Ok((vec![str1.to_string()], vec![str2.to_string()]))
    } else {
        let lcs_matrix = lcs_process(&rune_str1, &rune_str2);
        let diff = process_lcs_diff(&rune_str1, &rune_str2, &lcs_matrix, rune_str1.len(), rune_str2.len());
        Ok(diff)
    }
}

fn equal(a: &[char], b: &[char]) -> bool {
    a == b
}

fn lcs_process(rune_str1: &[char], rune_str2: &[char]) -> Vec<Vec<isize>> {
    let mut lcs_matrix = vec![vec![0; rune_str2.len() + 1]; rune_str1.len() + 1];

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
    if a > b { a } else { b }
}

fn process_lcs_diff(rune_str1: &[char], rune_str2: &[char], lcs_matrix: &[Vec<isize>], m: usize, n: usize) -> (Vec<String>, Vec<String>) {
    let mut diff = (vec![], vec![]);

    if m > 0 && n > 0 && rune_str1[m - 1] == rune_str2[n - 1] {
        diff = process_lcs_diff(rune_str1, rune_str2, lcs_matrix, m - 1, n - 1);
        diff.0.push(format!(" {}", rune_str1[m - 1]));
        diff.1.push("  ".to_string());
    } else if n > 0 && (m == 0 || lcs_matrix[m][n - 1] > lcs_matrix[m - 1][n]) {
        diff = process_lcs_diff(rune_str1, rune_str2, lcs_matrix, m, n - 1);
        diff.0.push(format!(" {}", rune_str2[n - 1]));
        diff.1.push(" +".to_string());
    } else if m > 0 && (n == 0 || lcs_matrix[m][n - 1] <= lcs_matrix[m - 1][n]) {
        diff = process_lcs_diff(rune_str1, rune_str2, lcs_matrix, m - 1, n);
        diff.0.push(format!(" {}", rune_str1[m - 1]));
        diff.1.push(" -".to_string());
    }

    diff
}
