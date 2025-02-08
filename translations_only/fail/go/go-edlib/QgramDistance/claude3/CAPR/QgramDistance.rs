
use std::collections::HashMap;

fn qgram_distance(str1: &str, str2: &str, split_length: usize) -> isize {
    let splitted_str1 = shingle(str1, split_length);
    let splitted_str2 = shingle(str2, split_length);

    let mut union = HashMap::new();
    for (key, count) in splitted_str1.iter() {
        union.insert(key.clone(), *count);
    }
    for (key, count) in splitted_str2.iter() {
        *union.entry(key.clone()).or_insert(0) += count;
    }

    let mut res = 0;

    for (key, _) in union.iter() {
        res += abs_diff(splitted_str1.get(key).unwrap_or(&0), splitted_str2.get(key).unwrap_or(&0)) as isize;
    }

    res
}

fn shingle(s: &str, k: usize) -> HashMap<Box<str>, usize> {
    let mut m = HashMap::new();
    if !s.is_empty() && k != 0 {
        let mut chars = s.chars();

        while let Some(first) = chars.next() {
            let mut shingle = String::new();
            shingle.push(first);

            for _ in 0..k-1 {
                if let Some(c) = chars.next() {
                    shingle.push(c);
                } else {
                    break;
                }
            }

            *m.entry(shingle.into_boxed_str()).or_insert(0) += 1;
        }
    }
    m
}

fn abs_diff(a: &usize, b: &usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}
