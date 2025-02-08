
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug)]
pub struct MoovIoAchAddenda99Contested {
    pub original_settlement_date: String,
}

impl MoovIoAchAddenda99Contested {
    pub fn original_settlement_date_field(&self) -> String {
        MoovIoAchConverters {}.string_field(&self.original_settlement_date, 3)
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let binding = moov_io_ach_string_zeros();
        binding.get(&(m as i32)).unwrap().to_string() + s
    }
}

fn moov_io_ach_string_zeros() -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..94 {
        out.insert(i, String::from_str("0").unwrap().repeat(i as usize));
    }
    out
}
