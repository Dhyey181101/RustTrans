
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| HashMap::new());

struct MoovIoAchBatchControl {
    entry_addenda_count: i32,
}

impl MoovIoAchBatchControl {
    fn entry_addenda_count_field(&self) -> String {
        numeric_field(self.entry_addenda_count, 6)
    }
}

struct MoovIoAchConverters;

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

