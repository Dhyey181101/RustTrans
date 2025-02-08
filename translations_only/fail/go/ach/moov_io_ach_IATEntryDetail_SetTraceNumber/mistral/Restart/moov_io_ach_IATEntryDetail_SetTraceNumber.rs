

use std::collections::HashMap;
use std::fmt;
use std::str;

const MAX_TRACE_NUMBER_LENGTH: usize = 15;

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
}

impl MoovIoAchIatEntryDetail {
    fn set_trace_number(&mut self, odfi_identification: &str, seq: i32) {
        let converters = MoovIoAchConverters;
        self.trace_number = converters
            .string_field(odfi_identification, 8)
            + &converters.numeric_field(seq, 7);
    }
}

struct Zeros {
    chars: String,
}

impl Zeros {
    fn new(n: usize) -> Self {
        Zeros {
            chars: "0".repeat(n),
        }
    }
}

impl fmt::Display for Zeros {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.chars)
    }
}

fn get_zeros(n: usize) -> Zeros {
    Zeros::new(n)
}

fn moov_io_ach_populate_map(max: i32, zero: char) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        let zeros = get_zeros((i + 1) as usize);
        out.insert(i, zeros.to_string());
    }
    out
}

fn main() {
    let mut iat_ed = MoovIoAchIatEntryDetail {
        trace_number: String::new(),
    };
    let mut converters = MoovIoAchConverters;
    iat_ed.set_trace_number("12345678", 123456);
    println!("{}", iat_ed.trace_number);
}

