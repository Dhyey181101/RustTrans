
fn process_lcs_diff(str1: &str, str2: &str, lcs_matrix: &Vec<Vec<isize>>, m: isize, n: isize) -> (String, String) {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    let mut diff: (String, String) = (String::new(), String::new());

    if m > 0 && n > 0 && rune_str1[(m - 1) as usize] == rune_str2[(n - 1) as usize] {
        let (diff1, diff2) = process_lcs_diff(str1, str2, lcs_matrix, m - 1, n - 1);
        diff.0 = format!("{} {}", diff1, rune_str1[(m - 1) as usize]);
        diff.1 = format!("{}  ", diff2);
        return diff;
    } else if n > 0 && (m == 0 || lcs_matrix[m as usize][n as usize] > lcs_matrix[(m - 1) as usize][n as usize]) {
        let (diff1, diff2) = process_lcs_diff(str1, str2, lcs_matrix, m, n - 1);
        diff.0 = format!("{} {}", diff1, rune_str2[(n - 1) as usize]);
        diff.1 = format!("{} +", diff2);
        return diff;
    } else if m > 0 && (n == 0 || lcs_matrix[m as usize][n as usize] <= lcs_matrix[(m - 1) as usize][n as usize]) {
        let (diff1, diff2) = process_lcs_diff(str1, str2, lcs_matrix, m - 1, n);
        diff.0 = format!("{} {}", diff1, rune_str1[(m - 1) as usize]);
        diff.1 = format!("{} -", diff2);
        return diff;
    }

    diff
}
