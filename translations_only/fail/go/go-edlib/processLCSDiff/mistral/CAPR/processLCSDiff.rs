
use std::isize;

pub fn process_lcs_diff(str1: String, str2: String, lcs_matrix: Vec<Vec<isize>>, m: isize, n: isize) -> Vec<String> {
    let rune_str1 = str1.chars().collect::<Vec<char>>();
    let rune_str2 = str2.chars().collect::<Vec<char>>();

    let mut diff = vec!["".to_string(); 2];

    if m > 0 && n > 0 && rune_str1[m as usize - 1] == rune_str2[n as usize - 1] {
        diff = process_lcs_diff(str1, str2, lcs_matrix, m - 1, n - 1);
        diff[0].push(' ');
        diff[0].push(rune_str1[m as usize - 1]);
        diff[1] += "  ";
        return diff;
    } else if n > 0 && (m == 0 || lcs_matrix[n as usize][m as usize - 1] > lcs_matrix[n as usize - 1][m as usize - 1]) {
        diff = process_lcs_diff(str1, str2, lcs_matrix, m, n - 1);
        diff[0].push(' ');
        diff[0].push(rune_str2[n as usize - 1]);
        diff[1] += " +";
        return diff;
    } else if m > 0 && (n == 0 || lcs_matrix[n as usize][m as usize - 1] <= lcs_matrix[n as usize - 1][m as usize - 1]) {
        diff = process_lcs_diff(str1, str2, lcs_matrix, m - 1, n);
        diff[0].push(' ');
        diff[0].push(rune_str1[m as usize - 1]);
        diff[1] += " -";
        return diff;
    }

    diff
}
