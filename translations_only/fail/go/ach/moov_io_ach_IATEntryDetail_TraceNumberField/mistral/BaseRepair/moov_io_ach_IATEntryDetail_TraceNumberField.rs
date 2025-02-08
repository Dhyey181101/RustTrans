

use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        let pad = Self::get_pad_string(m);
        format!("{}{}", pad, s)
    }

    fn get_pad_string(n: usize) -> String {
        let mut map = HashMap::new();
        for i in 0..=n {
            map.insert(i, "0".repeat(i));
        }
        map[&n].clone()
    }
}

#[derive(Default)]
struct MoovIoAchIatEntryDetail {
    trace_number: String,
    // ... other fields
}

impl MoovIoAchIatEntryDetail {
    fn trace_number_field(&self) -> String {
        self.string_field(&self.trace_number, 15)
    }

    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        let pad = MoovIoAchConverters::get_pad_string(m);
        format!("{}{}", pad.chars().rev().collect::<String>(), s)
    }
}

impl fmt::Display for MoovIoAchIatEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: {:<15} TraceNumber: {:<15}",
            self.trace_number_field(),
            self.trace_number
            // ... other fields
        )
    }
}

fn main() {
    let mut iat_entry = MoovIoAchIatEntryDetail::default();
    iat_entry.trace_number = "123456789".to_string();
    println!("{}", iat_entry);
}

