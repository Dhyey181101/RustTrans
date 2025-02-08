
use std::collections::HashMap;
use std::fmt::Write;

pub struct MoovIoAchFileControl {
    pub block_count: i32,
}

pub struct MoovIoAchConverters {}

impl MoovIoAchFileControl {
    pub fn block_count_field(&self) -> String {
        MoovIoAchConverters {}.numeric_field(self.block_count, 6)
    }
}

impl MoovIoAchConverters {
    pub fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[s.len() - max as usize..].to_string();
        } else {
            let m = max - s.len() as u32;
            let mut out = String::new();
            for _ in 0..m {
                write!(&mut out, "0").unwrap();
            }
            out.push_str(&s);
            return out;
        }
    }
}
