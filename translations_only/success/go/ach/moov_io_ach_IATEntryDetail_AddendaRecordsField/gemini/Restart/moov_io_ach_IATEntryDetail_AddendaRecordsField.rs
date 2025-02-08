
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct MoovIoAchIATEntryDetail {
    pub addenda_records: i32,
}

pub struct MoovIoAchConverters {}

impl MoovIoAchIATEntryDetail {
    pub fn addenda_records_field(&self) -> String {
        MoovIoAchConverters {}.numeric_field(self.addenda_records, 4)
    }
}

impl MoovIoAchConverters {
    pub fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() - max as usize)..].to_string();
        } else {
            let m = max as usize - s.len();
            let pad = moov_io_ach_string_zeros.get(&m).unwrap();
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
