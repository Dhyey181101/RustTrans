
use std::collections::HashMap;
use std::f32;

fn qgram_similarity(str1: &str, str2: &str, split_length: usize) -> f32 {
    let splitted_str1 = shingle(str1, split_length);
    let splitted_str2 = shingle(str2, split_length);
    let res = qgram_distance_custom_ngram(&splitted_str1, &splitted_str2) as f32;
    1.0 - (res / (splitted_str1.len() + splitted_str2.len()) as f32)
}

fn shingle(s: &str, k: usize) -> HashMap<String, isize> {
    let mut m = HashMap::new();
    if !s.is_empty() && k != 0 {
        let rune_s: Vec<char> = s.chars().collect();

        for i in 0..=rune_s.len().saturating_sub(k) {
            *m.entry(rune_s[i..i + k].iter().collect()).or_insert(0) += 1;
        }
    }
    m
}

fn qgram_distance_custom_ngram(splitted_str1: &HashMap<String, isize>, splitted_str2: &HashMap<String, isize>) -> isize {
    let mut union = HashMap::new();
    for i in splitted_str1.keys().chain(splitted_str2.keys()) {
        union.insert(i, 0);
    }

    let mut res = 0;
    for i in union.keys() {
        res += (splitted_str1.get(*i).unwrap_or(&0) - splitted_str2.get(*i).unwrap_or(&0)).abs();
    }

    res
}

fn main() {
    // Example usage
    println!("{}", qgram_similarity("hello", "hallo", 2));
}
