
use std::collections::HashMap;
use std::str::FromStr;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| HashMap::new());

struct MoovIoAchConverters;

struct MoovIoAchAdvEntryDetail {
    amount: i32,
}

impl MoovIoAchAdvEntryDetail {
    fn amount_field(&self) -> String {
        numeric_field(self.amount, 12)
    }
}

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    
    if s.len() as u32 > max {
        s[s.len()-max as usize..].to_string()
    } else {
        let m = (max - s.len() as u32) as usize;
        let pad = "0".repeat(m);
        format!("{}{}", pad, s)
    }
}

