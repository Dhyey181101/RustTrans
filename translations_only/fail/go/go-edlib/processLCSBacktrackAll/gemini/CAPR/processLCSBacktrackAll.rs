
use std::collections::HashMap;

pub fn process_lcs_backtrack_all(
    str1: &str,
    str2: &str,
    lcs_matrix: &Vec<Vec<isize>>,
    m: isize,
    n: isize,
) -> HashMap<String, ()> {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    let mut substrings: HashMap<String, ()> = HashMap::new();

    if m == 0 || n == 0 {
        substrings.insert(String::new(), ());
    } else if rune_str1[(m - 1) as usize] == rune_str2[(n - 1) as usize] {
        for key in process_lcs_backtrack_all(str1, str2, lcs_matrix, m - 1, n - 1).keys() {
            substrings.insert(key.to_string() + &rune_str1[(m - 1) as usize].to_string(), ());
        }
    } else {
        if lcs_matrix[(m - 1) as usize][n as usize] >= lcs_matrix[m as usize][(n - 1) as usize] {
            substrings.extend(
                process_lcs_backtrack_all(str1, str2, lcs_matrix, m - 1, n),
            );
        }
        if lcs_matrix[m as usize][(n - 1) as usize] >= lcs_matrix[(m - 1) as usize][n as usize] {
            substrings.extend(
                process_lcs_backtrack_all(str1, str2, lcs_matrix, m, n - 1),
            );
        }
    }

    substrings
}
