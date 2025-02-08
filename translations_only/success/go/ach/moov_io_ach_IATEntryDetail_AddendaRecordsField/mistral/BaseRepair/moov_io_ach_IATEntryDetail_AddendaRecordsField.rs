

use std::fmt;
use std::collections::HashMap;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            s[s.len() - max as usize..].to_string()
        } else {
            let m = (max - s.len() as u32) as usize;
            let pad = get_pad_string(m);
            format!("{}{}", pad, s)
        }
    }
}

fn get_pad_string(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, "0".repeat(i));
    }
    out[&n].to_string()
}

struct MoovIoAchIATEntryDetail {
    addenda_records: i32,
}

impl MoovIoAchIATEntryDetail {
    fn addenda_records_field(&self) -> String {
        MoovIoAchConverters.numeric_field(self.addenda_records, 4)
    }
}

fn main() {}

