

use std::collections::HashMap;
use std::iter;

fn ShingleSlice(s: &str, k: isize) -> Vec<String> {
    let mut out = Vec::new();
    if s.len() > 0 && k > 0 {
        let rune_s: Vec<char> = s.chars().collect();
        for i in 0..rune_s.len() - k as usize + 1 {
            let sub_str: String = rune_s[i..i + k as usize].iter().cloned().collect();
            out.push(sub_str);
        }
    }
    let mut m = HashMap::new();
    for k in out.iter() {
        *m.entry(k.clone()).or_insert(0) += 1;
    }
    let mut out_2 = Vec::new();
    for (k, _) in m {
        out_2.push(k);
    }
    out_2
}

