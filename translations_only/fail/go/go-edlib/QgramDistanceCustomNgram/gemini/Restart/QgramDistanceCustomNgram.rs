
use std::collections::HashMap;

fn qgram_distance_custom_ngram(splitted_str1: HashMap<String, isize>, splitted_str2: HashMap<String, isize>) -> isize {
    let mut union = HashMap::new();
    for (key, _) in splitted_str1.iter() {
        union.insert(key.clone(), 0);
    }
    for (key, _) in splitted_str2.iter() {
        union.insert(key.clone(), 0);
    }

    let mut res = 0;
    for (key, _) in union.iter() {
        res += (splitted_str1.get(key).unwrap_or(&0) - splitted_str2.get(key).unwrap_or(&0)).abs() as isize;
    }

    res
}
