

use std::collections::HashMap;
use std::fmt;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, _: (), s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        let pad = get_pad_string(m);
        pad.chars().chain(s.chars()).collect()
    }
}

fn get_pad_string(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, "0".repeat(i));
    }
    out[&n].to_string()
}

struct MoovIoAchAddenda99 {
    addenda_information: String,
}

impl MoovIoAchAddenda99 {
    fn new(addenda_information: String) -> Self {
        MoovIoAchAddenda99 {
            addenda_information,
        }
    }
}

