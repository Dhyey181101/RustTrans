

use std::collections::HashMap;
use std::fmt;
use std::str;

const MAX_DIGITS: usize = 94;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        let pad = Self::zeros(m);
        pad + s
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        }

        let m = (max - l) as usize;
        let pad = Self::zeros(m);
        pad + &s
    }

    fn zeros(n: usize) -> String {
        let mut out = String::new();
        for _ in 0..n {
            out.push('0');
        }
        out
    }
}

impl fmt::Display for MoovIoAchConverters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MoovIoAchConverters")
    }
}

struct MoovIoAchIATEntryDetail {
    trace_number: String,
    // ... other fields
}

impl MoovIoAchIATEntryDetail {
    fn set_trace_number(&mut self, odfi_identification: &str, seq: i32) {
        let converters = &MoovIoAchConverters; // create a reference to the singleton instance
        self.trace_number = Self::format_trace_number(odfi_identification, seq, converters);
    }

    fn format_trace_number(odfi_identification: &str, seq: i32, converters: &MoovIoAchConverters) -> String {
        let odfi_id_padded = converters.string_field(odfi_identification, 8);
        let seq_padded = converters.numeric_field(seq, 7);
        format!("{}{}", odfi_id_padded, seq_padded)
    }
}

fn main() {
    let mut iat_entry_detail = MoovIoAchIATEntryDetail {
        trace_number: String::new(),
        // ... other fields
    };

    let odfi_identification = "12345678";
    let seq = 123;
    iat_entry_detail.set_trace_number(odfi_identification, seq);

    println!("{}", iat_entry_detail.trace_number);
}

