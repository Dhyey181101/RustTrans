
use std::collections::HashMap;
use std::iter::once;

fn Shingle(s: &str, k: isize) -> HashMap<String, i32> {
    let mut m = HashMap::new();
    if s != "" && k != 0 {
        let rune_s: Vec<char> = s.chars().collect();

        for i in 0..rune_s.len().saturating_sub(k as usize) + 1 {
            let key = rune_s[i..i + k as usize].iter().collect::<String>();
            *m.entry(key).or_insert(0) += 1;
        }
    }
    m
}
