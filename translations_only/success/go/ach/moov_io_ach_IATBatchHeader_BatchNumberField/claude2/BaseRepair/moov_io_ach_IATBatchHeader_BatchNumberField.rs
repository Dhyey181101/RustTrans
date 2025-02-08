
use std::collections::HashMap;
use std::str::FromStr;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| HashMap::new());

struct MoovIoAchIatBatchHeader {
    batch_number: i32,
}

impl MoovIoAchIatBatchHeader {
    fn batch_number_field(&self) -> String {
        numeric_field(self.batch_number, 7)
    }
}

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    if s.len() as u32 > max {
        s[s.len() - max as usize..].to_string()
    } else {
        let m = max - s.len() as u32;
        let pad = "0".repeat(m as usize);
        format!("{}{}", pad, s)
    }
}

struct MoovIoAchConverters;

