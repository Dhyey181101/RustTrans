
use std::boxed::Box;

pub fn process_lcs_backtrack(
    str1: &str,
    str2: &str,
    lcs_matrix: &[&[isize]],
    m: usize,
    n: usize,
) -> String {
    let rune_str1 = str1.as_bytes();
    let rune_str2 = str2.as_bytes();

    let mut lcs = String::new();

    if m != 0 && n != 0 {
        let mut i = m;
        let mut j = n;

        while i > 0 && j > 0 {
            if rune_str1[i - 1] == rune_str2[j - 1] {
                lcs.insert(0, rune_str1[i - 1] as char);
                i -= 1;
                j -= 1;
            } else if lcs_matrix[i - 1][j] > lcs_matrix[i][j - 1] {
                i -= 1;
            } else {
                j -= 1;
            }
        }
    }

    lcs
}
