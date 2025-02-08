
use std::collections::HashMap;
use std::isize;

fn process_lcs_backtrack_all(str1: &str, str2: &str, lcs_matrix: &[Vec<isize>], m: isize, n: isize) -> HashMap<String, ()> {
    let rune_str1 = str1.chars().collect::<Vec<char>>();
    let rune_str2 = str2.chars().collect::<Vec<char>>();

    let mut substrings = HashMap::new();

    if m == 0 || n == 0 {
        substrings.insert(String::from(""), ());
    } else if rune_str1[m as usize - 1] == rune_str2[n as usize - 1] {
        for (key, _) in process_lcs_backtrack_all(str1, str2, lcs_matrix, m - 1, n - 1).iter() {
            let new_key = format!("{}{}", key, rune_str1[m as usize - 1]);
            substrings.insert(new_key, ());
        }
    } else {
        if lcs_matrix[(m - 1) as usize][n as usize] >= lcs_matrix[m as usize][(n - 1) as usize] {
            for (key, _) in process_lcs_backtrack_all(str1, str2, lcs_matrix, m - 1, n).iter() {
                substrings.insert(key.clone(), ());
            }
        }
        if lcs_matrix[m as usize][(n - 1) as usize] >= lcs_matrix[(m - 1) as usize][n as usize] {
            for (key, _) in process_lcs_backtrack_all(str1, str2, lcs_matrix, m, n - 1).iter() {
                substrings.insert(key.clone(), ());
            }
        }
    }

    substrings
}

fn main() {
    // Insert testcases here
}
