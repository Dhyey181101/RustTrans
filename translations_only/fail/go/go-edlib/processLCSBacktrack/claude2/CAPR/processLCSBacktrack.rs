
use std::str;

fn process_lcs_backtrack(str1: &str, str2: &str, lcs_matrix: &Vec<Vec<isize>>, m: usize, n: usize) -> String {
    let rune_str1 = str1.chars().collect::<Vec<_>>();
    let rune_str2 = str2.chars().collect::<Vec<_>>();

    if m == 0 || n == 0 {
        String::new()
    } else if rune_str1[m-1] == rune_str2[n-1] {
        let mut result = process_lcs_backtrack(str1, str2, lcs_matrix, m-1, n-1);
        result.push(rune_str1[m-1]);
        result
    } else if lcs_matrix[m][n-1] > lcs_matrix[m-1][n] {
        process_lcs_backtrack(str1, str2, lcs_matrix, m, n-1)
    } else {
        process_lcs_backtrack(str1, str2, lcs_matrix, m-1, n)
    }
}
