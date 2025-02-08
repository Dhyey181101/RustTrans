

use std::collections::HashMap;
use std::iter;

fn ShingleSlice(s: &str, k: isize) -> Vec<String> {
    let mut out = Vec::new();
    if s.len() > 0 && k > 0 {
        let rune_s: Vec<char> = s.chars().collect();
        let mut m = HashMap::new();
        for i in 0..(rune_s.len() - (k - 1) as usize + 1) {
            let key: String = rune_s[i..i + k as usize].iter().cloned().collect();
            let val = m.get(&key).unwrap_or(&0) + 1;
            m.insert(key, val);
        }
        out.extend(m.keys().cloned());
    }
    out
}

