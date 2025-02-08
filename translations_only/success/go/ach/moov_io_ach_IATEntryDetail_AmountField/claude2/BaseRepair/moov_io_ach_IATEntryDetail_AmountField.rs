
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| HashMap::new());

struct MoovIoAchIatEntryDetail {
    amount: i32,
}

impl MoovIoAchIatEntryDetail {
    fn amount_field(&self) -> String {
        numeric_field(self.amount, 10)
    }
}

struct MoovIoAchConverters;

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    if s.len() as u32 > max {
        s[s.len() - max as usize..].to_string()
    } else {
        let mut pad = String::new();
        for _ in 0..(max as i32 - s.len() as i32) {
            pad.push('0');
        }
        pad + &s
    }
}

