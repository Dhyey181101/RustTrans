
use std::collections::HashMap;

pub fn shingle_slice(s: &str, k: isize) -> Vec<String> {
    let mut out = Vec::new();
    let mut m = HashMap::new();
    if !s.is_empty() && k != 0 {
        let rune_s: Vec<char> = s.chars().collect();
        for i in 0..rune_s.len() - k as usize + 1 {
            let shingle = rune_s[i..i + k as usize].iter().collect::<String>();
            m.entry(shingle).and_modify(|e| *e += 1).or_insert(1);
        }
        for k in m.keys() {
            out.push(k.to_string());
        }
    }
    out
}
