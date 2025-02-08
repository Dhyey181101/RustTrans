
use std::collections::HashMap;

fn process_lcs_backtrack_all(str1: &str, str2: &str, lcs_matrix: Vec<Vec<isize>>, m: isize, n: isize) -> HashMap<String, ()> {
    let runestr1: Vec<char> = str1.chars().collect();
    let runestr2: Vec<char> = str2.chars().collect();

    let mut substrings = HashMap::new();

    if m == 0 || n == 0 {
        substrings.insert(String::new(), ());
    } else if runestr1[m as usize - 1] == runestr2[n as usize - 1] {
        for key in process_lcs_backtrack_all(str1, str2, lcs_matrix.clone(), m - 1, n - 1).keys() {
            substrings.insert(key.to_owned() + &runestr1[m as usize - 1].to_string(), ());
        }
    } else {
        if lcs_matrix[(m - 1) as usize][n as usize] >= lcs_matrix[m as usize - 1][n as usize - 1] {
            add_all(&mut substrings, &process_lcs_backtrack_all(str1, str2, lcs_matrix.clone(), m - 1, n));
        }
        if lcs_matrix[m as usize - 1][n as usize - 1] >= lcs_matrix[(m - 1) as usize][n as usize] {
            add_all(&mut substrings, &process_lcs_backtrack_all(str1, str2, lcs_matrix.clone(), m, n - 1));
        }
    }

    substrings
}

fn add_all(m: &mut HashMap<String, ()>, src_map: &HashMap<String, ()>) {
    for key in src_map.keys() {
        m.insert(key.to_owned(), ());
    }
}
