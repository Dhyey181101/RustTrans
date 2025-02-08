
use std::collections::HashMap;
use std::fmt::Write;

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
            let mut pad = String::new();
            for _ in 0..m {
                write!(&mut pad, "0").unwrap();
            }
            return format!("{}{}", pad, s);
        }
    }
}

pub fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out: HashMap<i32, String> = HashMap::new();
    for i in 0..max {
        let mut s = String::new();
        for _ in 0..i {
            write!(&mut s, "{}", zero).unwrap();
        }
        out.insert(i, s);
    }
    return out;
}
