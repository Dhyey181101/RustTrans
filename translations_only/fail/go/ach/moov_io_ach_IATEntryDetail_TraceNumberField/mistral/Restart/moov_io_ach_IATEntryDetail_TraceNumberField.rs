
use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = Self::get_pad_string(m);
        pad.chars().rev().chain(s.chars()).collect()
    }

    fn get_pad_string(n: usize) -> String {
        let mut map = HashMap::new();
        for i in 0..94 {
            map.insert(i, "0".repeat(i));
        }

        map.get(&n).unwrap_or(&"0".repeat(n)).to_string()
    }
}

#[derive(Default)]
struct MoovIoAchIatEntryDetail {
    trace_number: String,
}

impl MoovIoAchIatEntryDetail {
    fn trace_number_field(&self) -> String {
        self.string_field(&self.trace_number, 15)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = MoovIoAchConverters::get_pad_string(m);
        pad.chars().rev().chain(s.chars()).collect()
    }
}

impl fmt::Debug for MoovIoAchIatEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MoovIoAchIatEntryDetail")
            .field("trace_number", &self.trace_number)
            .finish()
    }
}

fn main() {
    let mut iat_entry = MoovIoAchIatEntryDetail::default();
    iat_entry.trace_number = "123456789".to_string();
    println!("{:?}", iat_entry);

    let padded_trace_number = iat_entry.trace_number_field();
    println!("{}", padded_trace_number);
}
