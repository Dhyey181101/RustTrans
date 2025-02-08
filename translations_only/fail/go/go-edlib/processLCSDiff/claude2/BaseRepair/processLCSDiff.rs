
use std::boxed::Box;

fn process_lcs_diff(str1: String, str2: String, lcs_matrix: Vec<Vec<isize>>, m: usize, n: usize) -> Box<[String]> {
    let mut rune_str1 = str1.chars().collect::<Vec<_>>();
    let mut rune_str2 = str2.chars().collect::<Vec<_>>();

    let mut diff = Box::new([String::new(), String::new()]);

    if m > 0 && n > 0 && rune_str1[m-1] == rune_str2[n-1] {
        let inner_diff = process_lcs_diff(str1, str2, lcs_matrix, m-1, n-1);
        diff[0].push_str(&format!(" {}", rune_str1[m-1]));
        diff[1].push_str("  ");
        diff
    } else if n > 0 && (m == 0 || lcs_matrix[m][n-1] > lcs_matrix[m-1][n]) {
        let inner_diff = process_lcs_diff(str1, str2, lcs_matrix, m, n-1);
        diff[0].push_str(&format!(" {}", rune_str2[n-1]));
        diff[1].push_str(" +");
        diff
    } else if m > 0 && (n == 0 || lcs_matrix[m][n-1] <= lcs_matrix[m-1][n]) {
        let inner_diff = process_lcs_diff(str1, str2, lcs_matrix, m-1, n);
        diff[0].push_str(&format!(" {}", rune_str1[m-1]));
        diff[1].push_str(" -");
        diff
    } else {
        diff
    }
}
