
use std::boxed::Box;

fn process_lcs_diff(str1: String, str2: String, lcs_matrix: Box<Vec<Vec<isize>>>, m: isize, n: isize) -> Box<[String; 2]> {
    let mut runestr1: Vec<char> = str1.chars().collect();
    let mut runestr2: Vec<char> = str2.chars().collect();

    let mut diff: Box<[String; 2]> = Box::new(["".to_string(), "".to_string()]);

    if m > 0 && n > 0 && runestr1[m as usize - 1] == runestr2[n as usize - 1] {
        diff = process_lcs_diff(str1, str2, lcs_matrix, m - 1, n - 1);
        diff[0] = format!("{} {}", diff[0], runestr1[m as usize - 1]);
        diff[1] = format!("{} {}", diff[1], " ");
        return diff;
    } else if n > 0 && (m == 0 || lcs_matrix[m as usize][n as usize - 1] > lcs_matrix[m as usize - 1][n as usize]) {
        diff = process_lcs_diff(str1, str2, lcs_matrix, m, n - 1);
        diff[0] = format!("{} {}", diff[0], runestr2[n as usize - 1]);
        diff[1] = format!("{} {}", diff[1], "+");
        return diff;
    } else if m > 0 && (n == 0 || lcs_matrix[m as usize][n as usize - 1] <= lcs_matrix[m as usize - 1][n as usize]) {
        diff = process_lcs_diff(str1, str2, lcs_matrix, m - 1, n);
        diff[0] = format!("{} {}", diff[0], runestr1[m as usize - 1]);
        diff[1] = format!("{} {}", diff[1], "-");
        return diff;
    }

    diff
}
