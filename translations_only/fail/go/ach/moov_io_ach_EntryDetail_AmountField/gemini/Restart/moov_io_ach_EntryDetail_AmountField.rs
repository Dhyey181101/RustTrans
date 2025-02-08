
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct MoovIoAchEntryDetail {
    pub amount: i32,
}

impl MoovIoAchEntryDetail {
    pub fn amount_field(&self) -> String {
        self.numeric_field(self.amount, 10)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() - max as usize)..].to_string();
        } else {
            let m = max - s.len() as u32;
            let pad = moov_io_ach_string_zeros.get(&m).unwrap();
            return pad.to_string() + &s;
        }
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() - max as usize)..].to_string();
        } else {
            let m = max - s.len() as u32;
            let pad = moov_io_ach_string_zeros.get(&m).unwrap();
            return pad.to_string() + &s;
        }
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<u32, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, "0".repeat(i as usize));
        }
        out
    };
}
