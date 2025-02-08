

use std::collections::HashMap;

fn process_lcs_backtrack_all(str1: String, str2: String, lcs_matrix: Vec<Vec<isize>>, m: usize, n: usize) -> HashMap<String, ()> {
    let runestr1: Vec<char> = str1.chars().collect();
    let runestr2: Vec<char> = str2.chars().collect();

    let mut substrings: HashMap<String, ()> = HashMap::new();

    if m == 0 || n == 0 {
        substrings.insert(String::new(), ());
    } else if runestr1[m - 1] == runestr2[n - 1] {
        for key in process_lcs_backtrack_all(str1[..m - 1].to_string(), str2[..n - 1].to_string(), lcs_matrix.clone(), m - 1, n - 1).keys() {
            substrings.insert(key.to_owned() + &runestr1[m - 1].to_string(), ());
        }
    } else {
        if lcs_matrix[m - 1][n] >= lcs_matrix[m][n - 1] {
            add_all(&mut substrings, &process_lcs_backtrack_all(str1[..m - 1].to_string(), str2.clone(), lcs_matrix.clone(), m - 1, n));
        }
        if lcs_matrix[m][n - 1] >= lcs_matrix[m - 1][n] {
            add_all(&mut substrings, &process_lcs_backtrack_all(str1.clone(), str2[..n - 1].to_string(), lcs_matrix.clone(), m, n - 1));
        }
    }

    substrings
}

fn add_all(m: &mut HashMap<String, ()>, src_map: &HashMap<String, ()>) {
    for key in src_map.keys() {
        m.insert(key.to_owned(), ());
    }
}

