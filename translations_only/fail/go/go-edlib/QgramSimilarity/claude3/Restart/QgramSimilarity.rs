
use std::collections::HashMap;

fn qgram_similarity(str1: &str, str2: &str, split_length: usize) -> f32 {
    let splitted_str1 = shingle(str1, split_length);
    let splitted_str2 = shingle(str2, split_length);
    let res = qgram_distance_custom_ngram(&splitted_str1, &splitted_str2) as f32;
    1.0 - (res / ((splitted_str1.len() + splitted_str2.len()) as f32))
}

fn shingle(s: &str, k: usize) -> HashMap<Box<str>, isize> {
    let mut m = HashMap::new();
    if !s.is_empty() && k != 0 {
        let mut chars = s.chars();
        let mut i = 0;
        while i < s.len() - k + 1 {
            let mut ngram = String::new();
            for _ in 0..k {
                ngram.push(chars.next().unwrap());
            }
            *m.entry(ngram.into_boxed_str()).or_insert(0) += 1;
            i += 1;
        }
    }
    m
}

fn qgram_distance_custom_ngram(
    splitted_str1: &HashMap<Box<str>, isize>,
    splitted_str2: &HashMap<Box<str>, isize>,
) -> isize {
    let mut union = HashMap::new();
    for i in splitted_str1.keys().chain(splitted_str2.keys()) {
        union.insert(i.clone(), 0);
    }

    let mut res = 0;
    for i in union.keys() {
        res += (splitted_str1.get(i).unwrap_or(&0) - splitted_str2.get(i).unwrap_or(&0)).abs();
    }

    res
}
