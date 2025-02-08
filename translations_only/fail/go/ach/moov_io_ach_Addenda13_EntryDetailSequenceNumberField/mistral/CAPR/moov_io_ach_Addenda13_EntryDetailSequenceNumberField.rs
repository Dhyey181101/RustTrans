

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &str = "0";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() as usize) - (max as usize)..].to_string();
        } else {
            let m = max as i32 - s.len() as i32;
            let pad = MoovIoAchAddenda13::get_pad_string(m);
            return pad + &s;
        }
    }
}

struct MoovIoAchAddenda13 {
    entry_detail_sequence_number: i32,
    _converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda13 {
    fn get_pad_string(n: i32) -> String {
        let mut out = HashMap::new();
        for i in 0..=n {
            out.insert(i, "0".repeat(i as usize));
        }
        return out.get(&n).unwrap().clone()
    }

    fn entry_detail_sequence_number_field(&self) -> String {
        return self._converters.numeric_field(self.entry_detail_sequence_number, 7);
    }
}

impl fmt::Display for MoovIoAchAddenda13 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.entry_detail_sequence_number_field())
    }
}

