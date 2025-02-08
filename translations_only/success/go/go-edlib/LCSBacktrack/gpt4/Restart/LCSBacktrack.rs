
fn lcs_backtrack(str1: &str, str2: &str) -> Result<String, String> {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    if rune_str1.is_empty() || rune_str2.is_empty() {
        Err("Can't process and backtrack any LCS with empty string".to_string())
    } else if equal(&rune_str1, &rune_str2) {
        Ok(str1.to_string())
    } else {
        Ok(process_lcs_backtrack(
            &rune_str1,
            &rune_str2,
            &lcs_process(&rune_str1, &rune_str2),
            rune_str1.len(),
            rune_str2.len(),
        ))
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
    if b > a { b } else { a }
}

fn process_lcs_backtrack(
    rune_str1: &[char],
    rune_str2: &[char],
    lcs_matrix: &[Vec<isize>],
    m: usize,
    n: usize,
) -> String {
    if m == 0 || n == 0 {
        "".to_string()
    } else if rune_str1[m - 1] == rune_str2[n - 1] {
        process_lcs_backtrack(rune_str1, rune_str2, lcs_matrix, m - 1, n - 1)
            + &rune_str1[m - 1].to_string()
    } else if lcs_matrix[m][n - 1] > lcs_matrix[m - 1][n] {
        process_lcs_backtrack(rune_str1, rune_str2, lcs_matrix, m, n - 1)
    } else {
        process_lcs_backtrack(rune_str1, rune_str2, lcs_matrix, m - 1, n)
    }
}
