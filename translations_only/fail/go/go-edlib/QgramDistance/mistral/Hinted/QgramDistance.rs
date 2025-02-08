

use std::collections::HashMap;
use std::isize;

fn QgramDistance(str1: &str, str2: &str, split_length: isize) -> isize {
    let splitted_str1 = Shingle(str1, split_length);
    let splitted_str2 = Shingle(str2, split_length);

    let mut union = HashMap::new();
    for _ in &splitted_str1 {
        union.insert(0, 0);
    }
    for _ in &splitted_str2 {
        union.insert(0, 0);
    }

    let res: isize = splitted_str1.iter()
        .zip(splitted_str2.iter())
        .map(|(s1, s2)| {
            let val = *union.get(&0).unwrap_or(&0);
            (*s1.0 - val).abs()
        })
        .sum();

    res
}

fn Shingle(s: &str, k: isize) -> HashMap<isize, isize> {
    let mut m = HashMap::new();
    if s != "" && k != 0 {
        let rune_s: Vec<char> = s.chars().collect();

        for i in 0..(rune_s.len() - k as usize + 1) {
            let key = i as isize;
            *m.entry(key).or_insert(0) += 1;
        }
    }
    m
}

