
use std::collections::HashSet;

fn process_lcs_backtrack_all(str1: &str, str2: &str, lcs_matrix: &Vec<Vec<isize>>, m: isize, n: isize) -> HashSet<String> {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();
    let mut substrings = HashSet::new();

    if m == 0 || n == 0 {
        substrings.insert("".to_string());
    } else if rune_str1[(m - 1) as usize] == rune_str2[(n - 1) as usize] {
        for key in process_lcs_backtrack_all(str1, str2, lcs_matrix, m - 1, n - 1).iter() {
            let mut new_key = key.clone();
            new_key.push(rune_str1[(m - 1) as usize]);
            substrings.insert(new_key);
        }
    } else {
        if lcs_matrix[(m - 1) as usize][n as usize] >= lcs_matrix[m as usize][(n - 1) as usize] {
            add_all(&mut substrings, &process_lcs_backtrack_all(str1, str2, lcs_matrix, m - 1, n));
        }
        if lcs_matrix[m as usize][(n - 1) as usize] >= lcs_matrix[(m - 1) as usize][n as usize] {
            add_all(&mut substrings, &process_lcs_backtrack_all(str1, str2, lcs_matrix, m, n - 1));
        }
    }

    substrings
}

fn add_all(dest: &mut HashSet<String>, src: &HashSet<String>) {
    for key in src {
        dest.insert(key.clone());
    }
}
