
use std::collections::HashMap;

use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| HashMap::new());

struct MoovIoAchAddenda99Contested {
    return_settlement_date: String,
}

impl MoovIoAchAddenda99Contested {
    fn return_settlement_date_field(&self) -> String {
        string_field(&self.return_settlement_date, 3)
    }
}

fn string_field(s: &str, max: u32) -> String {
    let ln = s.len() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let m = max - ln;
        let zeros = "0".repeat(m as usize);
        zeros + s
    }
}

struct MoovIoAchConverters;

