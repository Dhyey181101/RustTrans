
use std::collections::HashMap;

fn qgram_distance(str1: &str, str2: &str, split_length: usize) -> isize {
    let splitted_str1 = shingle(str1, split_length);
    let splitted_str2 = shingle(str2, split_length);

    let mut union = HashMap::new();
    for (k, v) in splitted_str1.iter() {
        union.insert(k.clone(), *v);
    }
    for (k, v) in splitted_str2.iter() {
        union.entry(k.clone()).or_insert(0).abs_diff(*v);
    }

    let mut res = 0;

    for (_, v) in union.iter() {
        res += *v as isize;
    }

    res
}

fn shingle(s: &str, k: usize) -> HashMap<Box<str>, usize> {
    let mut m = HashMap::new();
    if !s.is_empty() && k != 0 {
        let mut chars = s.chars();

        while let Some(chunk) = chars.as_str().get(..k) {
            let key = chunk.to_owned().into_boxed_str();
            *m.entry(key).or_insert(0) += 1;
            chars = chars.as_str()[1..].chars();
        }
    }
    m
}
