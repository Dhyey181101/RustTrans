
use std::collections::HashMap;

use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| HashMap::new());

struct MoovIoAchAddenda99Dishonored {
    return_settlement_date: String,
}

impl MoovIoAchAddenda99Dishonored {
    fn return_settlement_date_field(&self) -> String {
        string_field(&self.return_settlement_date, 3)
    }
}

struct MoovIoAchConverters;

fn string_field(s: &String, max: u32) -> String {
    let ln = s.len() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let m = (max - ln) as usize;
        let pad = "0".repeat(m);
        format!("{}{}", pad, s)
    }
}

