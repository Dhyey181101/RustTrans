

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

        pad.chars().chain(s.chars()).collect()
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;

        if l > max {
            return s[(l - max) as usize..].to_string();
        }

        let m = (max - l) as usize;
        let pad = get_zeros(m);

        pad.chars().chain(s.chars()).collect()
    }
}

fn get_zeros(n: usize) -> String {
    let mut zeros = HashMap::new();
    zeros.insert(1, "0".to_string());

    for i in 2..=n {
        let zero = "0".repeat(i);
        zeros.insert(i, zero);
    }

    zeros[&n].clone()
}

struct MoovIoAchIatEntryDetail {
    trace_number: String,
    converters: MoovIoAchConverters,
}

impl MoovIoAchIatEntryDetail {
    fn set_trace_number(&mut self, odfi_identification: &str, seq: i32) {
        let odfi_id_str = self.converters.string_field(odfi_identification, 8);
        let seq_str = self.converters.numeric_field(seq, 7);

        self.trace_number = format!("{}{}", odfi_id_str, seq_str);
    }
}

impl fmt::Display for MoovIoAchIatEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.trace_number)
    }
}

fn main() {
    let mut iat_entry_detail = MoovIoAchIatEntryDetail {
        trace_number: "".to_string(),
        converters: MoovIoAchConverters,
    };

    let odfi_identification = "12345678";
    let seq = 123456;

    iat_entry_detail.set_trace_number(odfi_identification, seq);

    println!("{}", iat_entry_detail);
}

