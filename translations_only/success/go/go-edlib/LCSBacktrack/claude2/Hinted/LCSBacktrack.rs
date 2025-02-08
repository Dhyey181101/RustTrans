
use std::cmp::max as std_max;
use std::convert::TryFrom;

fn lcs_backtrack(str1: &str, str2: &str) -> Result<String, Box<dyn std::error::Error>> {
    let rune_str1 = str1.chars().collect::<Vec<_>>();
    let rune_str2 = str2.chars().collect::<Vec<_>>();

    if rune_str1.is_empty() || rune_str2.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Can't process and backtrack any LCS with empty string"
        )));
    } else if equal(&rune_str1, &rune_str2) {
        return Ok(str1.to_string());
    }

    Ok(process_lcs_backtrack(
        str1,
        str2,
        lcs_process(&rune_str1, &rune_str2),
        rune_str1.len().try_into().unwrap(),
        rune_str2.len().try_into().unwrap(),
    ))
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

fn process_lcs_backtrack(
    str1: &str,
    str2: &str,
    lcs_matrix: Vec<Vec<isize>>,
    m: isize,
    n: isize,
) -> String {
    let rune_str1 = str1.chars().collect::<Vec<_>>();
    let rune_str2 = str2.chars().collect::<Vec<_>>();

    if m == 0 || n == 0 {
        return String::new();
    } else if rune_str1[m as usize - 1] == rune_str2[n as usize - 1] {
        let mut result = process_lcs_backtrack(str1, str2, lcs_matrix.clone(), m - 1, n - 1);
        result.push(rune_str1[m as usize - 1]);
        result
    } else if lcs_matrix[m as usize][(n - 1) as usize] > lcs_matrix[(m - 1) as usize][n as usize] {
        process_lcs_backtrack(str1, str2, lcs_matrix, m, n - 1)
    } else {
        process_lcs_backtrack(str1, str2, lcs_matrix, m - 1, n)
    }
}

