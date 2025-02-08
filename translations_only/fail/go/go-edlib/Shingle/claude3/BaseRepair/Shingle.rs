
use std::collections::HashMap;

fn Shingle(s: &str, k: isize) -> HashMap<String, isize> {
    let mut m = HashMap::new();
    if !s.is_empty() && k != 0 {
        let rune_s: Vec<char> = s.chars().collect();

        for i in 0..rune_s.len() - (k as usize) + 1 {
            let key: String = rune_s[i..i + (k as usize)].iter().collect();
            *m.entry(key).or_insert(0) += 1;
        }
    }
    m
}
