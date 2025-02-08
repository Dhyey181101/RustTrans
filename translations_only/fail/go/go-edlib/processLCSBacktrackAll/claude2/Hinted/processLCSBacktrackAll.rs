
use std::collections::HashMap;

type StringHashMap = HashMap<String, ()>;

fn process_lcs_backtrack_all(
    str1: &str,
    str2: &str,
    lcs_matrix: &Vec<Vec<isize>>,
    m: usize,
    n: usize,  
) -> StringHashMap {
    let rune_str1 = str1.chars().collect::<Vec<_>>();
    let rune_str2 = str2.chars().collect::<Vec<_>>();

    let mut substrings = HashMap::new();

    if m == 0 || n == 0 {
        substrings.insert(String::from(""), ());
    } else if rune_str1[m - 1] == rune_str2[n - 1] {
        for key in process_lcs_backtrack_all(str1, str2, lcs_matrix, m - 1, n - 1).keys() {
            let mut new_key = key.clone();
            new_key.push(rune_str1[m - 1]);
            substrings.insert(new_key, ());
        }
    } else {
        if lcs_matrix[m - 1][n] >= lcs_matrix[m][n - 1] {
            let mut substrings_temp = process_lcs_backtrack_all(str1, str2, lcs_matrix, m - 1, n);
            substrings.extend(substrings_temp);
        }
        if lcs_matrix[m][n - 1] >= lcs_matrix[m - 1][n] {
            let mut substrings_temp = process_lcs_backtrack_all(str1, str2, lcs_matrix, m, n - 1);
            substrings.extend(substrings_temp);
        }
    }

    substrings
}

fn main() {
    let str1 = "";
    let str2 = "";
    let lcs_matrix = vec![];
    let m = 0;
    let n = 0;

    let result = process_lcs_backtrack_all(str1, str2, &lcs_matrix, m, n);
    println!("Result: {:?}", result);  
}

