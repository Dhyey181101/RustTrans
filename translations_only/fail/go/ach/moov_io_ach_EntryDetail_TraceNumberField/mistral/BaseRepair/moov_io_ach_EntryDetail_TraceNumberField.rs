
use std::collections::HashMap;
use std::fmt;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = (max - ln) as usize;
            let pad = get_zeros(m);
            format!("{}{}", pad, s)
        }
    }
}

fn get_zeros(n: usize) -> String {
    let mut out = String::new();
    for _ in 0..=n {
        out.push('0');
    }
    out
}
