

use std::collections::HashMap;
use std::fmt;

const MAX_MAP_SIZE: usize = 94;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        let pad = get_zero_string(m);

        pad + &s
    }

    fn parse_string_field(&self, r: String) -> String {
        r.trim().to_string()
    }
}

struct MoovIoAchIATEntryDetail {
    rdfi_identification: String,
    check_digit: String,
    // ... other fields elided for brevity
}

impl MoovIoAchIATEntryDetail {
    fn set_rdfi(&mut self, rdfi: String) -> &MoovIoAchIATEntryDetail {
        let converter = MoovIoAchConverters; // create an instance of MoovIoAchConverters
        let s = converter.string_field(rdfi, 9);
        self.rdfi_identification = converter.parse_string_field(s[..8].to_string());
        self.check_digit = converter.parse_string_field(s[8..9].to_string());

        self
    }
}

struct ZeroString {
    s: String,
}

impl ZeroString {
    fn new(count: usize) -> ZeroString {
        ZeroString {
            s: "0".repeat(count),
        }
    }
}

impl fmt::Display for ZeroString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.s)
    }
}

fn get_zero_string(count: usize) -> String {
    let zero_string = ZeroString::new(count);

    format!("{}", zero_string)
}

fn populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();

    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }

    out
}

fn main() {
    let mut map = populate_map(MAX_MAP_SIZE, "0");
    let converters = MoovIoAchConverters;

    // ... other code elided for brevity
}

