

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
        let substrings_left = process_lcs_backtrack_all(str1, str2, lcs_matrix, m - 1, n);
        let substrings_upper = process_lcs_backtrack_all(str1, str2, lcs_matrix, m, n - 1);

        for (key, _) in substrings_left.iter() {
            substrings.insert(key.clone(), ());
        }
        for (key, _) in substrings_upper.iter() {
            substrings.insert(key.clone(), ());
        }
    }

    Rc::new(substrings)
}

fn add_all(m: &mut Rc<StringHashMap>, src_map: &Rc<StringHashMap>) {
    let mut m_map = HashMap::new();
    for (key, _) in src_map.iter() {
        m_map.insert(key.clone(), ());
    }
    *m = Rc::new(m_map);
}

