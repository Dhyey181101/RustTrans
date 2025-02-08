
use std::collections::HashMap;
use std::rc::Rc;

type StringHashMap = HashMap<String, ()>;

fn process_lcs_backtrack_all(
    str1: &str,
    str2: &str,
    lcs_matrix: &Vec<Vec<isize>>,
    m: usize,
    n: usize,
) -> Rc<StringHashMap> {
    let rune_str1 = str1.chars().collect::<Vec<char>>();
    let rune_str2 = str2.chars().collect::<Vec<char>>();

    let mut substrings = StringHashMap::new();

    if m == 0 || n == 0 {
        substrings.insert("".to_string(), ());
    } else if rune_str1[m - 1] == rune_str2[n - 1] {
        let substrings_ref = process_lcs_backtrack_all(str1, str2, lcs_matrix, m - 1, n - 1);
        for (key, _) in substrings_ref.iter() {
            let mut new_key = key.clone();
            new_key.push(rune_str1[m - 1]);
            substrings.insert(new_key, ());
        }
    } else {
        let mut substrings_1 = process_lcs_backtrack_all(str1, str2, lcs_matrix, m - 1, n);
        let mut substrings_2 = process_lcs_backtrack_all(str1, str2, lcs_matrix, m, n - 1);

        if lcs_matrix[m - 1][n] >= lcs_matrix[m][n - 1] {
            for (key, _) in substrings_1.iter() {
                substrings.insert(key.clone(), ());
            }
        }

        if lcs_matrix[m][n - 1] >= lcs_matrix[m - 1][n] {
            for (key, _) in substrings_2.iter() {
                substrings.insert(key.clone(), ());
            }
        }
    }

    Rc::new(substrings)
}

fn main() {
    // Example usage
}
