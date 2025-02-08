

use std::collections::HashMap;
use std::string;

struct MoovIoAchConverters;

const MOOV_IO_ACH_ZERO: &str = "0";

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, (0..i).map(|_| zero).collect::<String>());
    }
    out
}

impl MoovIoAchConverters {
    fn format_simple_date(&self, s: &string::String) -> String {
        if s.is_empty() {
            return self.string_field(s, 6);
        }
        s.clone()
    }

    fn string_field(&self, s: &String, max: u32) -> String {
        let ln = s.chars().count();
        if ln > max as usize {
            return s[..(max as usize)].to_string();
        }

        let m = (max as usize - ln) as usize;
        let pad = if let Some(pad) = MOOV_IO_ACH_ZERO.repeat(m).get(0..m) {
            pad.to_string()
        } else {
            "0".repeat(m)
        };

        pad + s
    }
}

