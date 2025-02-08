
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct MoovIoAchAdvFileControl {
    pub entry_hash: i32,
}

pub struct MoovIoAchConverters {}

impl MoovIoAchAdvFileControl {
    pub fn entry_hash_field(&self) -> String {
        MoovIoAchConverters {}.numeric_field(self.entry_hash, 10)
    }
}

impl MoovIoAchConverters {
    pub fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[s.len() - max as usize..].to_string();
        } else {
            let m = max - s.len() as u32;
            let pad = &MOOv_IO_ACH_STRING_ZEROS[&(m as i32)];
            return format!("{}{}", pad, s);
        }
    }
}

lazy_static! {
    static ref MOOv_IO_ACH_STRING_ZEROS: HashMap<i32, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, "0".repeat(i as usize));
        }
        out
    };
}
