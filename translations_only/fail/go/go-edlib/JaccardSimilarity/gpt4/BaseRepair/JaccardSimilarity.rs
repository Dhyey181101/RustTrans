
use std::collections::HashSet;

fn jaccard_similarity(str1: &str, str2: &str, split_length: isize) -> f32 {
    if str1.is_empty() || str2.is_empty() {
        return 0.0;
    }

    let splitted_str1: Vec<String>;
    let splitted_str2: Vec<String>;
    if split_length == 0 {
        splitted_str1 = str1.split_whitespace().map(String::from).collect();
        splitted_str2 = str2.split_whitespace().map(String::from).collect();
    } else {
        splitted_str1 = shingle_slice(str1, split_length);
        splitted_str2 = shingle_slice(str2, split_length);
    }

    let rune_str1: Vec<Vec<char>> = splitted_str1.iter().map(|s| s.chars().collect()).collect();
    let rune_str2: Vec<Vec<char>> = splitted_str2.iter().map(|s| s.chars().collect()).collect();

    let union_str = union(&splitted_str1, &splitted_str2);
    let jacc = (rune_str1.len() + rune_str2.len() - union_str.len()) as f32;

    jacc / union_str.len() as f32
}

fn shingle_slice(s: &str, k: isize) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut m: HashSet<String> = HashSet::new();
    if !s.is_empty() && k != 0 {
        let rune_s: Vec<char> = s.chars().collect();
        for i in 0..=rune_s.len() - k as usize {
            m.insert(rune_s[i..i + k as usize].iter().collect());
        }
        out = m.into_iter().collect();
    }
    out
}

fn union(a: &[String], b: &[String]) -> Vec<Vec<char>> {
    let mut m: HashSet<String> = a.iter().cloned().collect();
    let mut union_vec: Vec<String> = Vec::new();
    for item in a.iter().chain(b.iter()) {
        if m.insert(item.clone()) {
            union_vec.push(item.clone());
        }
    }
    union_vec.iter().map(|word| word.chars().collect()).collect()
}
