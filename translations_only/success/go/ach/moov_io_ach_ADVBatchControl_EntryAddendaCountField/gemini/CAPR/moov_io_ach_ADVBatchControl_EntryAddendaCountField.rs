
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct MoovIoAchAdvBatchControl {
    pub entry_addenda_count: i32,
}

pub struct MoovIoAchConverters {}

impl MoovIoAchAdvBatchControl {
    pub fn entry_addenda_count_field(&self) -> String {
        MoovIoAchConverters {}.numeric_field(self.entry_addenda_count, 6)
    }
}

impl MoovIoAchConverters {
    pub fn numeric_field(&self, n: i32, max: u32) -> String {
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
