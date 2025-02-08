
use std::collections::HashSet;

fn jaccard_similarity(str1: &str, str2: &str, split_length: isize) -> f32 {
    if str1.is_empty() || str2.is_empty() {
        return 0.0;
    }

    let splitted_str1: Vec<&str>;
    let splitted_str2: Vec<&str>;
    if split_length == 0 {
        splitted_str1 = str1.split_whitespace().collect();
        splitted_str2 = str2.split_whitespace().collect();
    } else {
        splitted_str1 = shingle_slice(str1, split_length);
        splitted_str2 = shingle_slice(str2, split_length);
    }

    let rune_str1: Vec<Vec<char>> = splitted_str1.iter().map(|&s| s.chars().collect()).collect();
    let rune_str2: Vec<Vec<char>> = splitted_str2.iter().map(|&s| s.chars().collect()).collect();

    let union_str = union(&splitted_str1, &splitted_str2);
    let jacc = (rune_str1.len() + rune_str2.len() - union_str.len()) as f32;

    jacc / union_str.len() as f32
}

fn shingle_slice(s: &str, k: isize) -> Vec<&str> {
    let mut out = Vec::new();
    let mut m = HashSet::new();
    if !s.is_empty() && k != 0 {
        let rune_s: Vec<char> = s.chars().collect();
        for i in 0..=rune_s.len() - k as usize {
            m.insert(&s[i..i + k as usize]);
        }
        for &k in m.iter() {
            out.push(k);
        }
    }
    out
}

fn union(a: &[&str], b: &[&str]) -> Vec<Vec<char>> {
    let mut m = HashSet::new();
    let mut result = Vec::new();
    for &item in a.iter().chain(b.iter()) {
        if m.insert(item) {
            result.push(item.chars().collect());
        }
    }
    result
}
