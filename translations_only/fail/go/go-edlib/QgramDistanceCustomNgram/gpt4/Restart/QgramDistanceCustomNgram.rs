
use std::collections::HashMap;
use std::collections::HashSet;

fn qgram_distance_custom_ngram(splitted_str1: &HashMap<String, isize>, splitted_str2: &HashMap<String, isize>) -> isize {
    let mut union_keys = HashSet::new();
    for key in splitted_str1.keys() {
        union_keys.insert(key);
    }
    for key in splitted_str2.keys() {
        union_keys.insert(key);
    }

    let mut res = 0;
    for key in union_keys {
        let val1 = splitted_str1.get(key).unwrap_or(&0);
        let val2 = splitted_str2.get(key).unwrap_or(&0);
        res += (*val1 - *val2).abs();
    }

    res
}
