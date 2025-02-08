
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0"));

pub struct MoovIoAchAddenda13 {
    pub entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda13 {
    pub fn entry_detail_sequence_number_field(&self) -> String {
        MoovIoAchConverters {}.numeric_field(self.entry_detail_sequence_number, 7)
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&(m as usize)).unwrap();
            return format!("{}{}", pad, s);
        }
    }
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}
