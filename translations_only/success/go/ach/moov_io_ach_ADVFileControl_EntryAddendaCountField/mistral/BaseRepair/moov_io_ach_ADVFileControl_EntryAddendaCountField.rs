

use std::collections::HashMap;
use std::fmt;
use std::str;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() as u32) as usize - (max as usize)..].to_string();
        } else {
            let m = max as i32 - s.len() as i32;
            let pad = get_pad_string(m as usize);
            return pad + &s;
        }
    }
}

fn get_pad_string(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, "0".repeat(i));
    }
    return out[&n].clone();
}

struct MoovIoAchAdvFileControl {
    entry_addenda_count: i32,
}

impl MoovIoAchAdvFileControl {
    fn entry_addenda_count_field(&self) -> String {
        let converters = Box::new(MoovIoAchConverters);
        return converters.numeric_field(self.entry_addenda_count, 8);
    }
}

fn main() {}

