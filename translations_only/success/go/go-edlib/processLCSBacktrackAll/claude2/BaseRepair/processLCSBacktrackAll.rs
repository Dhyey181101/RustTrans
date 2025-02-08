

use std::collections::HashMap;

fn process_lcs_backtrack_all(str1: &str, str2: &str, lcs_matrix: &Vec<Vec<isize>>, m: usize, n: usize) -> StringHashMap {
    let rune_str1 = str1.as_bytes();
    let rune_str2 = str2.as_bytes();

    let mut substrings = StringHashMap::new();

    if m == 0 || n == 0 {
        substrings.insert(String::from(""), ());
    } else if rune_str1[m-1] == rune_str2[n-1] {
        for key in process_lcs_backtrack_all(str1, str2, lcs_matrix, m-1, n-1).keys() {
            substrings.insert(key.to_owned() + &str1.as_bytes()[m-1].to_string(), ());
        }
    } else {
        if lcs_matrix[m-1][n] >= lcs_matrix[m][n-1] {
            let mut temp = process_lcs_backtrack_all(str1, str2, lcs_matrix, m-1, n);
            append_all(&mut substrings, temp);
        }
        if lcs_matrix[m][n-1] >= lcs_matrix[m-1][n] {
            let mut temp = process_lcs_backtrack_all(str1, str2, lcs_matrix, m, n-1);
            append_all(&mut substrings, temp);  
        }
    }

    substrings
}

fn append_all(target: &mut StringHashMap, src: StringHashMap) {
    for (key, _) in src {
        target.insert(key, ());
    }
}

// StringHashMap is HashMap substitute for string  
type StringHashMap = HashMap<String, ()>;

