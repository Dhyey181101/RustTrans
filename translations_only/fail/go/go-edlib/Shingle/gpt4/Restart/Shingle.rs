
use std::collections::HashMap;

fn shingle(s: &str, k: isize) -> HashMap<String, isize> {
    let mut m = HashMap::new();
    if !s.is_empty() && k != 0 {
        let rune_s: Vec<char> = s.chars().collect();
        let k_usize = k as usize;

        for i in 0..=rune_s.len().saturating_sub(k_usize) {
            *m.entry(rune_s[i..i + k_usize].iter().collect()).or_insert(0) += 1;
        }
    }
    m
}
