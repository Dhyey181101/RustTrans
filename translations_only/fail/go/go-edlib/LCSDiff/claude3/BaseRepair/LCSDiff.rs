
use std::cmp::max;

fn lcs_diff(str1: &str, str2: &str) -> Result<Vec<String>, &'static str> {
    let runestr1: Vec<char> = str1.chars().collect();
    let runestr2: Vec<char> = str2.chars().collect();

    if runestr1.is_empty() || runestr2.is_empty() {
        return Err("Can't process LCS diff with empty string");
    } else if equal(&runestr1, &runestr2) {
        return Ok(vec![str1.to_string()]);
    }

    let diff = process_lcs_diff(
        str1,
        str2,
        lcs_process(&runestr1, &runestr2),
        runestr1.len(),
        runestr2.len(),
    );
    Ok(diff)
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
    let mut lcs_matrix: Vec<Vec<isize>> = vec![vec![0; runestr2.len() + 1]; runestr1.len() + 1];

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

fn process_lcs_diff(
    str1: &str,
    str2: &str,
    lcs_matrix: Vec<Vec<isize>>,
    m: usize,
    n: usize,
) -> Vec<String> {
    let runestr1: Vec<char> = str1.chars().collect();
    let runestr2: Vec<char> = str2.chars().collect();

    let mut diff: Vec<String> = vec![String::new(); 2];

    if m > 0 && n > 0 && runestr1[m - 1] == runestr2[n - 1] {
        let mut temp = process_lcs_diff(str1, str2, lcs_matrix, m - 1, n - 1);
        temp[0].push(' ');
        temp[0].push(runestr1[m - 1]);
        temp[1].push_str("  ");
        diff = temp;
    } else if n > 0 && (m == 0 || lcs_matrix[m][n - 1] >= lcs_matrix[m - 1][n]) {
        let mut temp = process_lcs_diff(str1, str2, lcs_matrix, m, n - 1);
        temp[0].push(' ');
        temp[0].push(runestr2[n - 1]);
        temp[1].push_str(" +");
        diff = temp;
    } else if m > 0 && (n == 0 || lcs_matrix[m][n - 1] < lcs_matrix[m - 1][n]) {
        let mut temp = process_lcs_diff(str1, str2, lcs_matrix, m - 1, n);
        temp[0].push(' ');
        temp[0].push(runestr1[m - 1]);
        temp[1].push_str(" -");
        diff = temp;
    }

    diff
}
