
use std::boxed::Box;

pub fn process_lcs_backtrack(
    str1: &str,
    str2: &str,
    lcs_matrix: &[Vec<isize>],
    m: usize,
    n: usize,
) -> String {
    let rune_str1 = str1.as_bytes();
    let rune_str2 = str2.as_bytes();

    if m == 0 || n == 0 {
        return String::new();
    } else if rune_str1[m - 1] as char == rune_str2[n - 1] as char {
        return format!(
            "{}{}",
            process_lcs_backtrack(
                str1,
                str2,
                lcs_matrix,
                m.saturating_sub(1),
                n.saturating_sub(1)
            ),
            rune_str1[m - 1] as char
        );
    } else if lcs_matrix[m][n - 1] > lcs_matrix[m - 1][n] {
        return process_lcs_backtrack(str1, str2, lcs_matrix, m, n - 1);
    }

    process_lcs_backtrack(str1, str2, lcs_matrix, m.saturating_sub(1), n)
}
