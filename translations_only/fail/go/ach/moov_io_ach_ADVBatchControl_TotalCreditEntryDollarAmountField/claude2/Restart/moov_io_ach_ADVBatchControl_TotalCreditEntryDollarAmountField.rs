
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| HashMap::new());

struct MoovIoAchAdVBatchControl {
    total_credit: i32,
}

impl MoovIoAchAdVBatchControl {
    fn total_credit_entry_dollar_amount_field(&self) -> String {
        numeric_field(self.total_credit, 20)
    }
}

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    if s.len() as u32 > max {
        s[s.len() - max as usize..].to_string()
    } else {
        let m = max - s.len() as u32;
        let pad = "0".repeat(m as usize);
        pad + &s
    }
}

struct MoovIoAchConverters;

