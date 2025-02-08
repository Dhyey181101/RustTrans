

use std::collections::HashMap;
use std::fmt;
use std::str;

const MAX_LENGTH: usize = 94;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        let pad = get_zeros(m);
        format!("{}{}", pad, s)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        }

        let m = (max - l) as usize;
        let pad = get_zeros(m);
        format!("{}{}", pad, s)
    }
}

struct MoovIoAchIatEntryDetail {
    trace_number: String,
    converters: MoovIoAchConverters,
}

impl MoovIoAchIatEntryDetail {
    fn new() -> Self {
        MoovIoAchIatEntryDetail {
            trace_number: String::new(),
            converters: MoovIoAchConverters,
        }
    }

    fn set_trace_number(&mut self, odfi_identification: &str, seq: i32) {
        self.trace_number = (
            self.converters.string_field(odfi_identification, 8) + &self.converters.numeric_field(seq, 7)
        );
    }
}

impl fmt::Display for MoovIoAchIatEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.trace_number)
    }
}

fn get_zeros(n: usize) -> String {
    let mut zeros = HashMap::new();
    for i in 0..MAX_LENGTH {
        zeros.insert(i, "0".repeat(i));
    }
    zeros.get(&n).unwrap().to_string()
}

fn main() {
    let mut iat_ed = MoovIoAchIatEntryDetail::new();
    let odfi_identification = "12345678";
    let seq = 123;
    iat_ed.set_trace_number(odfi_identification, seq);
    println!("{}", iat_ed);
}

