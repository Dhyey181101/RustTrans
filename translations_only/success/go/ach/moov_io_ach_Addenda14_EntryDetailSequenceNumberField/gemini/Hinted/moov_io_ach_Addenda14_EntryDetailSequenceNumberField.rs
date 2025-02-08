
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda14 {
    pub entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda14 {
    pub fn entry_detail_sequence_number_field(&self) -> String {
        let s = self.numeric_field(self.entry_detail_sequence_number, 7);
        s
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() - max as usize)..].to_string();
        } else {
            let m = max as usize - s.len();
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap();
            return format!("{}{}", pad, s);
        }
    }
}

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = {
        let mut out = HashMap::with_capacity(94);
        for i in 0..94 {
            out.insert(i, "0".repeat(i));
        }
        out
    };
}
