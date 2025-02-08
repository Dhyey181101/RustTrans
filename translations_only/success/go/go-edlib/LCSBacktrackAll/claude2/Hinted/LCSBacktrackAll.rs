
use std::collections::HashMap;

fn lcs_backtrack_all(str1: &str, str2: &str) -> Result<Vec<String>, &'static str> {
    let rune_str1 = str1.chars().collect::<Vec<_>>();
    let rune_str2 = str2.chars().collect::<Vec<_>>();

    if rune_str1.is_empty() || rune_str2.is_empty() {
        return Err("Can't process and backtrack any LCS with empty string");
    } else if equal(&rune_str1, &rune_str2) {
        return Ok(vec![str1.to_string()]);
    }

    Ok(process_lcs_backtrack_all(str1, str2, lcs_process(&rune_str1, &rune_str2), rune_str1.len(), rune_str2.len()))
}

fn equal(a: &[char], b: &[char]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b).all(|(x, y)| x == y)
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
    if b > a { b } else { a }
}

fn process_lcs_backtrack_all(str1: &str, str2: &str, lcs_matrix: Vec<Vec<isize>>, m: usize, n: usize) -> Vec<String> {
    let rune_str1 = str1.chars().collect::<Vec<_>>();
    let rune_str2 = str2.chars().collect::<Vec<_>>();

    let mut substrings = HashMap::new();

    if m == 0 || n == 0 {
        substrings.insert(String::new(), ());
    } else if rune_str1[m - 1] == rune_str2[n - 1] {
        for key in process_lcs_backtrack_all(str1, str2, lcs_matrix.clone(), m - 1, n - 1) {
            substrings.insert(key + &rune_str1[m - 1].to_string(), ()); 
        }
    } else {
        if lcs_matrix[m - 1][n] >= lcs_matrix[m][n - 1] {
            let keys = process_lcs_backtrack_all(str1, str2, lcs_matrix.clone(), m - 1, n);
            for key in keys {
                substrings.insert(key, ());
            }
        }
        if lcs_matrix[m][n - 1] >= lcs_matrix[m - 1][n] {
            let keys = process_lcs_backtrack_all(str1, str2, lcs_matrix, m, n - 1);
            for key in keys {
                substrings.insert(key, ());
            }
        }
    }

    substrings.into_keys().map(|s| s.to_owned()).collect()
}

