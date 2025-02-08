
use std::collections::HashMap;
use std::cmp;

fn qgram_distance(str1: &str, str2: &str, split_length: usize) -> isize {
    let splitted_str1 = shingle(str1, split_length);
    let splitted_str2 = shingle(str2, split_length);

    let mut union = HashMap::new();
    for key in splitted_str1.keys().chain(splitted_str2.keys()) {
        union.insert(key, 0);
    }

    let mut res = 0;

    for key in union.keys() {
        let count1 = splitted_str1.get(*key).unwrap_or(&0);
        let count2 = splitted_str2.get(*key).unwrap_or(&0);
        res += (*count1 as isize - *count2 as isize).abs();
    }

    res
}

fn shingle(s: &str, k: usize) -> HashMap<&str, isize> {
    let mut m = HashMap::new();
    if !s.is_empty() && k != 0 {
        let rune_s: Vec<char> = s.chars().collect();

        for i in 0..=cmp::max(0, rune_s.len() as isize - k as isize) as usize {
            *m.entry(&s[i..i + k]).or_insert(0) += 1;
        }
    }
    m
}
