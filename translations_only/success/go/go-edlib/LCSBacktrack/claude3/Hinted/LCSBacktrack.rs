
use std::cmp::max;

fn lcs_backtrack(str1: &str, str2: &str) -> Result<String, &'static str> {
    let runestr1: Vec<char> = str1.chars().collect();
    let runestr2: Vec<char> = str2.chars().collect();

    if runestr1.is_empty() || runestr2.is_empty() {
        return Err("Can't process and backtrack any LCS with empty string");
    } else if equal(&runestr1, &runestr2) {
        return Ok(str1.to_string());
    }

    let lcs_matrix = lcs_process(&runestr1, &runestr2);
    Ok(process_lcs_backtrack(
        str1,
        str2,
        Box::new(lcs_matrix),
        runestr1.len(),
        runestr2.len(),
    ))
}

fn equal(a: &[char], b: &[char]) -> bool {
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

fn lcs_process(runestr1: &[char], runestr2: &[char]) -> Vec<Vec<isize>> {
    let mut lcs_matrix = vec![vec![0; runestr2.len() + 1]; runestr1.len() + 1];

    for i in 1..=runestr1.len() {
        for j in 1..=runestr2.len() {
            if runestr1[i - 1] == runestr2[j - 1] {
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
    lcs_matrix: Box<Vec<Vec<isize>>>,
    m: usize,
    n: usize,
) -> String {
    let runestr1: Vec<char> = str1.chars().collect();
    let runestr2: Vec<char> = str2.chars().collect();

    if m == 0 || n == 0 {
        return String::new();
    } else if runestr1[m - 1] == runestr2[n - 1] {
        process_lcs_backtrack(str1, str2, lcs_matrix, m - 1, n - 1)
            + &String::from(runestr1[m - 1])
    } else if lcs_matrix[m][n - 1] > lcs_matrix[m - 1][n] {
        process_lcs_backtrack(str1, str2, lcs_matrix, m, n - 1)
    } else {
        process_lcs_backtrack(str1, str2, lcs_matrix, m - 1, n)
    }
}
