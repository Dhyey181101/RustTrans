
use std::str;

struct IATEntryDetail {
    trace_number: String,
}

impl IATEntryDetail {
    fn set_trace_number(&mut self, odfi_identification: String, seq: i32) {
        self.trace_number = string_field(&odfi_identification, 8) + &numeric_field(seq, 7);
    }
}

fn string_field(s: &str, max: u32) -> String {
    let ln = s.len() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let mut pad = String::with_capacity(max as usize);
        pad.extend(std::iter::repeat('0').take(max as usize - ln as usize));
        pad + s
    }
}

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    let l = s.len() as u32;
    if l > max {
        s[s.len() - max as usize..].to_string()
    } else {
        let mut pad = String::with_capacity(max as usize);
        pad.extend(std::iter::repeat('0').take(max as usize - l as usize));
        pad + &s
    }
}

struct Converters;

