
fn process_lcs_backtrack(str1: String, str2: String, lcs_matrix: Vec<Vec<isize>>, m: usize, n: usize) -> String {
    let mut result = String::new();
    let runestr1: Vec<char> = str1.chars().collect();
    let runestr2: Vec<char> = str2.chars().collect();

    if m == 0 || n == 0 {
        return result;
    } else if runestr1[m - 1] == runestr2[n - 1] {
        result = process_lcs_backtrack(str1, str2, lcs_matrix, m - 1, n - 1) + &runestr1[m - 1].to_string();
        return result;
    } else if lcs_matrix[m][n - 1] > lcs_matrix[m - 1][n] {
        result = process_lcs_backtrack(str1, str2, lcs_matrix, m, n - 1);
    } else {
        result = process_lcs_backtrack(str1, str2, lcs_matrix, m - 1, n);
    }

    result
}
