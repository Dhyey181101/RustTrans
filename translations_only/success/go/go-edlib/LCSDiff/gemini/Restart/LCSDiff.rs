
use std::cmp::max;

fn lcs_diff(str1: &str, str2: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    if rune_str1.len() == 0 || rune_str2.len() == 0 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Can't process LCS diff with empty string",
        )));
    } else if rune_str1 == rune_str2 {
        return Ok(vec![str1.to_string()]);
    }

    let diff = process_lcs_diff(
        str1,
        str2,
        lcs_process(&rune_str1, &rune_str2),
        rune_str1.len(),
        rune_str2.len(),
    );
    Ok(diff)
}

fn equal(a: &[char], b: &[char]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }
    true
}

fn lcs_process(rune_str1: &[char], rune_str2: &[char]) -> Vec<Vec<isize>> {
    let mut lcs_matrix: Vec<Vec<isize>> = vec![vec![0; rune_str2.len() + 1]; rune_str1.len() + 1];
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

fn process_lcs_diff(
    str1: &str,
    str2: &str,
    lcs_matrix: Vec<Vec<isize>>,
    m: usize,
    n: usize,
) -> Vec<String> {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    let mut diff: Vec<String> = vec![String::new(), String::new()];

    if m > 0 && n > 0 && rune_str1[m - 1] == rune_str2[n - 1] {
        diff = process_lcs_diff(str1, str2, lcs_matrix, m - 1, n - 1);
        diff[0].push_str(&format!(" {}", rune_str1[m - 1]));
        diff[1].push_str("  ");
        diff
    } else if n > 0 && (m == 0 || lcs_matrix[m][n - 1] > lcs_matrix[m - 1][n]) {
        diff = process_lcs_diff(str1, str2, lcs_matrix, m, n - 1);
        diff[0].push_str(&format!(" {}", rune_str2[n - 1]));
        diff[1].push_str(" +");
        diff
    } else if m > 0 && (n == 0 || lcs_matrix[m][n - 1] <= lcs_matrix[m - 1][n]) {
        diff = process_lcs_diff(str1, str2, lcs_matrix, m - 1, n);
        diff[0].push_str(&format!(" {}", rune_str1[m - 1]));
        diff[1].push_str(" -");
        diff
    } else {
        diff
    }
}
