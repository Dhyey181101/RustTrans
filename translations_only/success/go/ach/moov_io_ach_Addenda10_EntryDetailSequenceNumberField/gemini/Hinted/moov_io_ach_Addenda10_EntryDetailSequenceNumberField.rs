
use std::collections::HashMap;
use std::fmt::Write;
use std::str::FromStr;
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct MoovIoAchAddenda10 {
    pub entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda10 {
    pub fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[s.len() - max as usize..].to_string();
        } else {
            let m = max - s.len() as u32;
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap();
            return format!("{}{}", pad, s);
        }
    }
}

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<u32, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, "0".repeat(i as usize));
        }
        out
    };
}
