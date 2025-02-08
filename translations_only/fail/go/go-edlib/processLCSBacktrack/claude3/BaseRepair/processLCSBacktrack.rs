
fn process_lcs_backtrack(str1: &str, str2: &str, lcs_matrix: Vec<Vec<isize>>, m: usize, n: usize) -> String {
    let runestr1: Vec<char> = str1.chars().collect();
    let runestr2: Vec<char> = str2.chars().collect();

    if m == 0 || n == 0 {
        return String::new();
    } else if runestr1[m - 1] == runestr2[n - 1] {
        let mut result = process_lcs_backtrack(str1, str2, lcs_matrix, m - 1, n - 1);
        result.push(runestr1[m - 1]);
        return result;
    } else if lcs_matrix[m][n - 1] > lcs_matrix[m - 1][n] {
        return process_lcs_backtrack(str1, str2, lcs_matrix, m, n - 1);
    }

    process_lcs_backtrack(str1, str2, lcs_matrix, m - 1, n)
}
