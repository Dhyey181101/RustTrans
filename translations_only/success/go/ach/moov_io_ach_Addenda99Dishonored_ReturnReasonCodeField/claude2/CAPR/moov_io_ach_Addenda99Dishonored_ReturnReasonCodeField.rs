
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| HashMap::new());

struct MoovIoAchAddenda99Dishonored {
    return_reason_code: String,
}

impl MoovIoAchAddenda99Dishonored {
    fn return_reason_code_field(&self) -> String {
        string_field(&self.return_reason_code, 2)
    }
}

fn string_field(s: &str, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let m = (max - ln) as usize;
        let mut result = String::from("0");
        for _ in 0..m {
            result.push('0');
        }
        result + s
    }
}

