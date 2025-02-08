
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct MoovIoAchAdvBatchControl {
    pub batch_number: i32,
}

pub struct MoovIoAchConverters {}

impl MoovIoAchAdvBatchControl {
    pub fn batch_number_field(&self) -> String {
        MoovIoAchConverters {}.numeric_field(self.batch_number, 7)
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
            let pad = MOOv_IO_ACH_STRING_ZEROS.get(&(m as usize)).unwrap();
            return format!("{}{}", pad, s);
        }
    }
}

lazy_static! {
    static ref MOOv_IO_ACH_STRING_ZEROS: HashMap<usize, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, "0".repeat(i));
        }
        out
    };
}
