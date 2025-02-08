
use std::collections::HashMap;

pub fn shingle_slice(s: &str, k: isize) -> Vec<String> {
    let mut out = Vec::new();
    let mut m = HashMap::new();
    if !s.is_empty() && k != 0 {
        let rune_s = s.chars().collect::<Vec<char>>();
        for i in 0..rune_s.len() - k as usize + 1 {
            let shingle = rune_s[i..i + k as usize].iter().collect::<String>();
            let count = m.entry(shingle).or_insert(0);
            *count += 1;
        }
        for (k, _) in m {
            out.push(k.to_string());
        }
    }
    out
}
