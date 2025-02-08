
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda14 {
    pub entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda14 {
    pub fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }

    fn numeric_field<T: Display>(&self, n: T, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() - max as usize)..].to_string();
        } else {
            let m = max - s.len() as u32;
            let pad = moov_io_ach_string_zeros.get(&m).unwrap();
            return format!("{}{}", pad, s);
        }
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<u32, String> =
        moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<u32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i as u32, zero.repeat(i as usize));
    }
    out
}
