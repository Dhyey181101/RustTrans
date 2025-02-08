
use std::collections::HashMap;

fn qgram_similarity(str1: &str, str2: &str, split_length: isize) -> f32 {
    let splitted_str1 = shingle(str1, split_length);
    let splitted_str2 = shingle(str2, split_length);
    let res = qgram_distance_custom_ngram(&splitted_str1, &splitted_str2) as f32;
    1.0 - (res / (splitted_str1.len() + splitted_str2.len()) as f32)  
}

fn shingle(s: &str, k: isize) -> HashMap<String, isize> {
    let mut m = HashMap::new();
    if !s.is_empty() && k != 0 {
        let rune_s: Vec<char> = s.chars().collect();
        for i in 0..rune_s.len() - k as usize + 1 {
            *m.entry(rune_s[i..i+k as usize].iter().collect()).or_insert(0) += 1;
        }
    }
    m
}

fn qgram_distance_custom_ngram(splitted_str1: &HashMap<String, isize>, splitted_str2: &HashMap<String, isize>) -> isize {
    let mut splitted_str1 = splitted_str1.clone();
    for (key, _) in splitted_str2 {
        splitted_str1.entry(key.clone()).or_insert(0);
    }
    let mut res = 0;
    for (key, _) in &splitted_str1 {
        res += (splitted_str1[key] - splitted_str2.get(key).unwrap_or(&0)).abs();
    }
    res  
}
