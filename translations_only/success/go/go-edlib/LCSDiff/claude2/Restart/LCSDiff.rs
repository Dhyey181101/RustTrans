
use std::cmp::max as std_max;
use std::error::Error;

fn lcs_diff(str1: &str, str2: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let rune_str1 = str1.as_bytes();
    let rune_str2 = str2.as_bytes();

    if rune_str1.is_empty() || rune_str2.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Can't process LCS diff with empty string"
        )));
    } else if equal(rune_str1, rune_str2) {
        return Ok(vec![str1.to_owned()]);
    }

    let diff = process_lcs_diff(str1, str2, lcs_process(&rune_str1, &rune_str2), rune_str1.len(), rune_str2.len());
    Ok(diff)
}

fn equal(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b).all(|(x, y)| *x == *y)
}

fn lcs_process(rune_str1: &[u8], rune_str2: &[u8]) -> Vec<Vec<isize>> {
    let mut lcs_matrix = vec![vec![0; rune_str2.len() + 1]; rune_str1.len() + 1];

    for i in 1..=rune_str1.len() {
        for j in 1..=rune_str2.len() {
            if rune_str1[i - 1] == rune_str2[j - 1] {
                lcs_matrix[i][j] = lcs_matrix[i - 1][j - 1] + 1;
            } else {
                lcs_matrix[i][j] = my_max(lcs_matrix[i][j - 1], lcs_matrix[i - 1][j]);
            }
        }
    }

    lcs_matrix
}

fn my_max(a: isize, b: isize) -> isize {
    if b > a { b } else { a }
}

fn process_lcs_diff(str1: &str, str2: &str, lcs_matrix: Vec<Vec<isize>>, m: usize, n: usize) -> Vec<String> {
    let mut diff = vec![String::new(), String::new()];

    if m > 0 && n > 0 && str1.as_bytes()[m - 1] == str2.as_bytes()[n - 1] {
        let mut rec_diff = process_lcs_diff(str1, str2, lcs_matrix.clone(), m - 1, n - 1);
        let c = str1.as_bytes()[m - 1] as char;
        rec_diff[0].push(' ');
        rec_diff[0].push(c);
        rec_diff[1].push_str("  ");
        rec_diff
    } else if n > 0 && (m == 0 || lcs_matrix[m][n - 1] > lcs_matrix[m - 1][n]) {
        let mut rec_diff = process_lcs_diff(str1, str2, lcs_matrix.clone(), m, n - 1);
        let c = str2.as_bytes()[n - 1] as char;
        rec_diff[0].push(' ');
        rec_diff[0].push(c);
        rec_diff[1].push_str(" +");
        rec_diff
    } else if m > 0 && (n == 0 || lcs_matrix[m][n - 1] <= lcs_matrix[m - 1][n]) {
        let mut rec_diff = process_lcs_diff(str1, str2, lcs_matrix, m - 1, n);
        let c = str1.as_bytes()[m - 1] as char;
        rec_diff[0].push(' ');
        rec_diff[0].push(c);
        rec_diff[1].push_str(" -");
        rec_diff
    } else {
        diff
    }
}

