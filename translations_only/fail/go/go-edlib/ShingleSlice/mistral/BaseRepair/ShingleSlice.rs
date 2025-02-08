
use std::collections::HashMap;
use std::iter;

fn ShingleSlice(s: &str, k: isize) -> Vec<String> {
    let mut out = Vec::new();
    if s.len() > 0 && k > 0 {
        let rune_s: Vec<char> = s.chars().collect();
        for i in 0..rune_s.len() - k as usize + 1 {
            let slice: String = rune_s[i..i + k as usize].iter().collect();
            let count = out.iter().filter(|x| x == &&slice).count();
            if count == 0 {
                out.push(slice);
            }
        }
    }
    out
}
