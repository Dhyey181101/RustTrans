
fn process_lcs_backtrack(str1: &str, str2: &str, lcs_matrix: &Vec<Vec<isize>>, m: isize, n: isize) -> String {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    if m == 0 || n == 0 {
        return "".to_string();
    } else if rune_str1[(m - 1) as usize] == rune_str2[(n - 1) as usize] {
        return process_lcs_backtrack(str1, str2, lcs_matrix, m - 1, n - 1) + &rune_str1[(m - 1) as usize].to_string();
    } else if lcs_matrix[m as usize][n as usize - 1] > lcs_matrix[m as usize - 1][n as usize] {
        return process_lcs_backtrack(str1, str2, lcs_matrix, m, n - 1);
    }

    process_lcs_backtrack(str1, str2, lcs_matrix, m - 1, n)
}
