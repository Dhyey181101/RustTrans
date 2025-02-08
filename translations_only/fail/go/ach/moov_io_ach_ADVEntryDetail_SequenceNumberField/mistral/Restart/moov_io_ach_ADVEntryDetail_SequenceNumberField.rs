
use std::fmt;
use std::collections::HashMap;
use std::boxed::Box;

const ZEROS: &str = "0";

struct MoovIoAchAdvEntryDetail {
    sequence_number: i32,
    // ... other fields ...
}

impl MoovIoAchAdvEntryDetail {
    fn sequence_number_field(&self) -> String {
        MoovIoAchConverters::numeric_field(&MoovIoAchConverters, self.sequence_number, 4)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(converters: &MoovIoAchConverters, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            let start_index = (l - max) as usize;
            let substring = s[start_index..].to_string();
            substring
        } else {
            let m = max - l;
            let pad = get_pad_string(m as usize);
            format!("{}{}", pad, s)
        }
    }
}

fn get_pad_string(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, "0".repeat(i));
    }
    out[&n].clone()
}

impl fmt::Display for MoovIoAchAdvEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.sequence_number_field())
    }
}
