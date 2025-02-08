
use std::collections::HashMap;

use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

struct MoovIoAchAddenda99Dishonored {
    return_reason_code: String,
}

impl MoovIoAchAddenda99Dishonored {
    fn return_reason_code_field(&self) -> String {
        let max = 2;
        string_field(&self.return_reason_code, max)
    }
}

fn string_field(s: &String, max: usize) -> String {
    let ln = s.len();
    if ln > max {
        s[..max].to_string()
    } else {
        let m = max - ln;
        if let Some(pad) = MOOV_IO_ACH_STRINGZEROS.get(&m) {
            format!("{}{}", pad, s)
        } else {
            format!("{}{}", "0".repeat(m), s)
        }
    }
}

fn populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out
}

