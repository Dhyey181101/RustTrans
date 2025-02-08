
use std::collections::VecDeque;

fn process_lcs_backtrack(str1: &str, str2: &str, lcs_matrix: &Vec<Vec<isize>>, m: isize, n: isize) -> String {
    let mut result = VecDeque::new();

    let mut i = m;
    let mut j = n;

    while i > 0 && j > 0 {
        if str1.chars().nth(i as usize - 1).unwrap() == str2.chars().nth(j as usize - 1).unwrap() {
            result.push_front(str1.chars().nth(i as usize - 1).unwrap());
            i -= 1;
            j -= 1;
        } else if lcs_matrix[i as usize][j as usize - 1] > lcs_matrix[i as usize - 1][j as usize] {
            j -= 1;
        } else {
            i -= 1;
        }
    }

    result.iter().collect()
}
