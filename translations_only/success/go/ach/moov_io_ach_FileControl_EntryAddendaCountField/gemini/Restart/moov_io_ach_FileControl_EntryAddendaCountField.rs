
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Write;

pub struct MoovIoAchFileControl {
    pub entry_addenda_count: i32,
}

pub struct MoovIoAchConverters;

impl MoovIoAchFileControl {
    pub fn entry_addenda_count_field(&self) -> String {
        MoovIoAchConverters {}.numeric_field(self.entry_addenda_count, 8)
    }
}

impl MoovIoAchConverters {
    pub fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = moov_io_ach_string_zeros.get(&(m as usize)).unwrap();
            return format!("{}{}", pad, s);
        }
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<usize, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, "0".repeat(i));
        }
        out
    };
}
