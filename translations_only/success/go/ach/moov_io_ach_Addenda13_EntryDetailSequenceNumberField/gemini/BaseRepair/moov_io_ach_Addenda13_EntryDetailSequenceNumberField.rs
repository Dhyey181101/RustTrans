
use once_cell::sync::Lazy;
use std::collections::HashMap;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| populate_map(94, "0"));

pub struct MoovIoAchAddenda13 {
    entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda13 {
    pub fn entry_detail_sequence_number_field(&self) -> String {
        numeric_field(self.entry_detail_sequence_number, 7)
    }
}

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    if s.len() as u32 > max {
        return s[(s.len() - max as usize)..].to_string();
    } else {
        let m = (max - s.len() as u32) as usize;
        match MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            Some(pad) => return pad.clone() + &s,
            None => return "0".repeat(m) + &s,
        }
    }
}

fn populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}
