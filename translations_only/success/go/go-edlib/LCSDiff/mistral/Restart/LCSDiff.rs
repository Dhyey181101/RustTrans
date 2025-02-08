
use std::cmp::max;
use std::error::Error;

pub fn lcs_diff(str1: &str, str2: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let rune_str1 = str1.chars().collect::<Vec<char>>();
    let rune_str2 = str2.chars().collect::<Vec<char>>();

    if rune_str1.is_empty() || rune_str2.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Can't process LCS diff with empty string",
        )));
    } else if are_equal(&rune_str1, &rune_str2) {
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

fn are_equal(a: &[char], b: &[char]) -> bool {
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

fn lcs_process(rune_str1: &[char], rune_str2: &[char]) -> Vec<Vec<i32>> {
    let mut lcs_matrix: Vec<Vec<i32>> = vec![vec![0; rune_str2.len() + 1]; rune_str1.len() + 1];

    for i in 1..rune_str1.len() + 1 {
        for j in 1..rune_str2.len() + 1 {
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
    lcs_matrix: Vec<Vec<i32>>,
    m: usize,
    n: usize,
) -> Vec<String> {
    let rune_str1 = str1.chars().collect::<Vec<char>>();
    let rune_str2 = str2.chars().collect::<Vec<char>>();

    let mut diff = vec!["".to_string(); 2];

    if m > 0 && n > 0 && rune_str1[m - 1] == rune_str2[n - 1] {
        diff = process_lcs_diff(str1, str2, lcs_matrix.clone(), m - 1, n - 1);
        diff[0].push(' ');
        diff[0].push(rune_str1[m - 1]);
        diff[1].push_str("  ");
        return diff;
    } else if n > 0 && (m == 0 || lcs_matrix[m][n - 1] > lcs_matrix[m - 1][n]) {
        diff = process_lcs_diff(str1, str2, lcs_matrix.clone(), m, n - 1);
        diff[0].push(' ');
        diff[0].push(rune_str2[n - 1]);
        diff[1].push_str(" +");
        return diff;
    } else if m > 0 && (n == 0 || lcs_matrix[m][n - 1] <= lcs_matrix[m - 1][n]) {
        diff = process_lcs_diff(str1, str2, lcs_matrix.clone(), m - 1, n);
        diff[0].push(' ');
        diff[0].push(rune_str1[m - 1]);
        diff[1].push_str(" -");
        return diff;
    }

    diff
}
