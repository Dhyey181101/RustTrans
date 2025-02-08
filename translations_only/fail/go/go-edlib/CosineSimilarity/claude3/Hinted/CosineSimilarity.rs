
use std::collections::HashMap;

fn cosine_similarity(str1: &str, str2: &str, split_length: isize) -> f32 {
    if str1.is_empty() || str2.is_empty() {
        return 0.0;
    }

    let (splitted_str1, splitted_str2) = if split_length == 0 {
        (
            str1.split_whitespace().map(String::from).collect::<Vec<_>>(),
            str2.split_whitespace().map(String::from).collect::<Vec<_>>(),
        )
    } else {
        (
            shingle_slice(str1, split_length as usize),
            shingle_slice(str2, split_length as usize),
        )
    };

    let runestr1: Vec<Vec<char>> = splitted_str1.iter().map(|s| s.chars().collect()).collect();
    let runestr2: Vec<Vec<char>> = splitted_str2.iter().map(|s| s.chars().collect()).collect();

    let mut l1 = Vec::new();
    let mut l2 = Vec::new();

    let union_str: Vec<Vec<char>> = union(&splitted_str1, &splitted_str2);

    for word in &union_str {
        l1.push(if find(&runestr1, word) != -1 { 1 } else { 0 });
        l2.push(if find(&runestr2, word) != -1 { 1 } else { 0 });
    }

    let mut cosine_sim: f32 = 0.0;
    for i in 0..union_str.len() {
        cosine_sim += l1[i] as f32 * l2[i] as f32;
    }

    cosine_sim / (sum(&l1) as f32 * sum(&l2) as f32).sqrt()
}

fn shingle_slice(s: &str, k: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut m = HashMap::new();

    if !s.is_empty() && k != 0 {
        for i in 0..s.len() - k + 1 {
            let shingle = &s[i..i + k];
            *m.entry(shingle.to_string()).or_insert(0) += 1;
        }
        out = m.keys().map(String::from).collect();
    }

    out
}

fn union(a: &[String], b: &[String]) -> Vec<Vec<char>> {
    let mut m = HashMap::new();

    for item in a.iter().chain(b.iter()) {
        m.insert(item, true);
    }

    m.keys()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect()
}

fn find(slice: &[Vec<char>], val: &[char]) -> isize {
    for (i, item) in slice.iter().enumerate() {
        if equal(item, val) {
            return i as isize;
        }
    }
    -1
}

fn equal(a: &[char], b: &[char]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for (i, v) in a.iter().enumerate() {
        if v != &b[i] {
            return false;
        }
    }
    true
}

fn sum(arr: &[isize]) -> isize {
    arr.iter().sum()
}
