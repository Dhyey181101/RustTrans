

use std::collections::HashMap;

fn process_lcs_backtrack_all(str1: &str, str2: &str, lcs_matrix: &Vec<Vec<isize>>, m: usize, n: usize) -> StringHashMap {
    let rune_str1 = str1.as_bytes();
    let rune_str2 = str2.as_bytes();

    let mut substrings = StringHashMap::new();

    if m == 0 || n == 0 {
        substrings.insert(String::from(""), ());
    } else if rune_str1[m-1] == rune_str2[n-1] {
        for key in process_lcs_backtrack_all(str1, str2, lcs_matrix, m-1, n-1).keys() {
            let mut new_key = key.clone();
            new_key.push(rune_str1[m-1] as char);
            substrings.insert(new_key, ());  
        }
    } else {
        if lcs_matrix[m-1][n] >= lcs_matrix[m][n-1] {
            let mut temp = process_lcs_backtrack_all(str1, str2, lcs_matrix, m-1, n);
            append_string_hashmap(&mut substrings, &mut temp);
        }
        if lcs_matrix[m][n-1] >= lcs_matrix[m-1][n] {
             let mut temp = process_lcs_backtrack_all(str1, str2, lcs_matrix, m, n-1);
             append_string_hashmap(&mut substrings, &mut temp);
        }
    }

    substrings
}

// StringHashMap is HashMap substitute for string  
type StringHashMap = HashMap<String, ()>;

fn append_string_hashmap(map1: &mut StringHashMap, map2: &mut StringHashMap) {
    for (key, _) in map2.drain() {
        map1.insert(key, ());
    }
}

