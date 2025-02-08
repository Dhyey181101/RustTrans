
use std::collections::HashMap;

fn qgram_distance_custom_ngram(
    mut splitted_str1: HashMap<String, isize>,
    mut splitted_str2: HashMap<String, isize>,
) -> isize {
    let mut union = HashMap::new();
    for i in splitted_str1.keys() {
        union.insert(i.to_string(), 0);
    }
    for i in splitted_str2.keys() {
        union.insert(i.to_string(), 0);
    }

    let mut res = 0;
    for (_i, _) in union {
        res += splitted_str1
            .get(&_i)
            .unwrap_or(&0)
            .abs_diff(*splitted_str2.get(&_i).unwrap_or(&0)) as isize;
    }

    res
}

