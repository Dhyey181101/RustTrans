
use std::collections::HashMap;
use std::fmt::Write;

pub struct MoovIoAchAddenda05 {
    pub entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda05 {
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
            let pad = MOOv_IO_ACH_STRING_ZEROS.get(&m).unwrap();
            return format!("{}{}", pad, s);
        }
    }
}

lazy_static::lazy_static! {
    static ref MOOv_IO_ACH_STRING_ZEROS: HashMap<u32, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, "0".repeat(i as usize));
        }
        out
    };
}
