

use std::fmt;
use std::collections::HashMap;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() as usize) as usize - (max as usize) ..].to_string();
        } else {
            let m = max as i32 - s.len() as i32;
            let pad = get_zeros(m as usize);
            return pad + &s;
        }
    }
}

fn get_zeros(n: usize) -> String {
    let mut out = String::new();
    for _ in 0 .. n {
        out.push('0');
    }
    out
}

struct MoovIoAchAddenda16 {
    entry_detail_sequence_number: i32,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda16 {
    fn entry_detail_sequence_number_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.entry_detail_sequence_number, 7)
    }
}

fn main() {
    let mut map = HashMap::new();
    for i in 0 .. 94 {
        let zero = "0".to_string();
        let value = get_zeros(i);
        map.insert(i, value);
    }
}

