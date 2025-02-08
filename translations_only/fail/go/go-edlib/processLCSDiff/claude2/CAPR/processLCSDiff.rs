
use std::boxed::Box;

fn process_lcs_diff(str1: &String, str2: &String, lcs_matrix: Vec<Vec<isize>>, m: usize, n: usize) -> Box<[String]> {
    let rune_str1 = str1.as_bytes();
    let rune_str2 = str2.as_bytes();

    let mut diff: Box<[String]> = Box::new([String::new(), String::new()]);

    if m > 0 && n > 0 && rune_str1[m-1] == rune_str2[n-1] {
        diff = process_lcs_diff(str1, str2, lcs_matrix.clone(), m-1, n-1);
        diff[0].push_str(&String::from_utf8(vec![rune_str1[m-1]]).unwrap());
        diff[0].push_str(" ");
        diff[1].push_str("  ");
        diff
    } else if n > 0 && (m == 0 || lcs_matrix[m][n-1] > lcs_matrix[m-1][n]) {
        diff = process_lcs_diff(str1, str2, lcs_matrix.clone(), m, n-1);
        diff[0].push_str(&String::from_utf8(vec![rune_str2[n-1]]).unwrap());
        diff[0].push_str(" ");
        diff[1].push_str(" +");
        diff
    } else if m > 0 && (n == 0 || lcs_matrix[m][n-1] <= lcs_matrix[m-1][n]) {
        diff = process_lcs_diff(str1, str2, lcs_matrix.clone(), m-1, n);
        diff[0].push_str(&String::from_utf8(vec![rune_str1[m-1]]).unwrap());
        diff[0].push_str(" ");
        diff[1].push_str(" -");
        diff
    } else {
        diff
    }
}

