

use std::collections::HashMap;
use std::str;

const ZERO: &str = "0";

struct MoovIoAchFileHeader {
    immediate_origin: String,
    // ... other fields and structs
    moov_io_ach_converters: MoovIoAchConverters,
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

struct MoovIoAchConverters;

impl MoovIoAchFileHeader {
    fn immediate_origin_field(&self) -> String {
        if self.immediate_origin.is_empty() {
            " ".repeat(10)
        } else {
            let trimmed = self.immediate_origin.trim();
            if self.validate_opts.is_some() && self.validate_opts.as_ref().unwrap().bypass_origin_validation && trimmed.len() == 10 {
                trimmed.to_string()
            } else {
                format!(" {}", self.string_field(trimmed, 9))
            }
        }
    }

    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..(max as usize)].to_string()
        } else {
            let m = (max - ln) as usize;
            let pad = MoovIoAchConverters::get_zero_string(m);
            format!("{}{}", pad, s)
        }
    }
}

impl MoovIoAchConverters {
    fn get_zero_string(n: usize) -> String {
        let mut map: HashMap<usize, String> = HashMap::new();
        for i in 0..100 {
            map.insert(i, "0".repeat(i));
        }
        map.get(&n).cloned().unwrap_or(String::from(""))
    }
}

struct MoovIoAchValidateOpts {
    bypass_origin_validation: bool,
    // ... other fields and structs
}

fn main() {
    // ... code using the structs and functions
}

