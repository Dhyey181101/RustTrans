
use std::collections::VecDeque;

pub fn process_lcs_backtrack(str1: &str, str2: &str, lcs_matrix: &Vec<Vec<isize>>, m: isize, n: isize) -> String {
    let mut str1_chars: Vec<char> = str1.chars().collect();
    let mut str2_chars: Vec<char> = str2.chars().collect();

    if m == 0 || n == 0 {
        return String::new();
    } else if str1_chars[m as usize - 1] == str2_chars[n as usize - 1] {
        return process_lcs_backtrack(str1, str2, lcs_matrix, m - 1, n - 1) + &str1_chars[m as usize - 1].to_string();
    } else if lcs_matrix[m as usize][n as usize - 1] > lcs_matrix[m as usize - 1][n as usize] {
        return process_lcs_backtrack(str1, str2, lcs_matrix, m, n - 1);
    }

    process_lcs_backtrack(str1, str2, lcs_matrix, m - 1, n)
}
