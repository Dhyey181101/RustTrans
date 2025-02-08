
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Write;

pub struct MoovIoAchAdvFileControl {
    pub batch_count: i32,
}

pub struct MoovIoAchConverters {}

impl MoovIoAchAdvFileControl {
    pub fn batch_count_field(&self) -> String {
        MoovIoAchConverters {}.numeric_field(self.batch_count, 6)
    }
}

impl MoovIoAchConverters {
    pub fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[s.len() - max as usize..].to_string();
        } else {
            let m = max - s.len() as u32;
            let pad = moov_io_ach_string_zeros.get(&m).unwrap();
            return format!("{}{}", pad, s);
        }
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<u32, String> = moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<u32, String> {
    let mut out: HashMap<u32, String> = HashMap::new();
    for i in 0..max {
        out.insert(i as u32, zero.repeat(i as usize));
    }
    out
}
