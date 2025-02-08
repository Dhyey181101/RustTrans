
use std::collections::HashMap;

fn shingle_slice(s: &str, k: isize) -> Vec<String> {
    let mut out = Vec::new();
    let mut m = HashMap::new();

    if !s.is_empty() && k != 0 {
        let rune_s: Vec<char> = s.chars().collect();
        for i in 0..rune_s.len() - (k as usize) + 1 {
            let shingle: String = rune_s[i..i + (k as usize)].iter().collect();
            *m.entry(shingle).or_insert(0) += 1;
        }
        for k in m.keys() {
            out.push(k.to_string());
        }
    }

    out
}
