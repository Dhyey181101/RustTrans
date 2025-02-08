
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct MoovIoAchAdvEntryDetail {
    pub julian_day: i32,
}

pub struct MoovIoAchConverters {}

impl MoovIoAchAdvEntryDetail {
    pub fn julian_date_day_field(&self) -> String {
        MoovIoAchConverters {}.numeric_field(self.julian_day, 3)
    }
}

impl MoovIoAchConverters {
    pub fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[s.len() - max as usize..].to_string();
        } else {
            let m = max - s.len() as u32;
            let pad = &moov_io_ach_string_zeros.get(&(m as i32)).unwrap();
            return format!("{}{}", pad, s);
        }
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<i32, String> = moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}
