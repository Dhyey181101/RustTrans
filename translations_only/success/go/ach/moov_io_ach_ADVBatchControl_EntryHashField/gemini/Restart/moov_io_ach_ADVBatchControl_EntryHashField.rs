
use lazy_static::lazy_static;

pub struct MoovIoAchAdvBatchControl {
    pub entry_hash: i32,
}

pub struct MoovIoAchConverters {}

impl MoovIoAchAdvBatchControl {
    pub fn entry_hash_field(&self) -> String {
        let converters = MoovIoAchConverters {};
        converters.numeric_field(self.entry_hash, 10)
    }
}

impl MoovIoAchConverters {
    pub fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[s.len() - max as usize..].to_string();
        } else {
            let m = max - s.len() as u32;
            let pad = moov_io_ach_string_zeros.get(m as usize).unwrap();
            return format!("{}{}", pad, s);
        }
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: Vec<String> = {
        let mut out = Vec::new();
        for i in 0..94 {
            out.push("0".repeat(i as usize));
        }
        out
    };
}
