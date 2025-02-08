

use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

struct MoovIoAchAddenda99Contested {
    contested_return_code: String,
}

impl MoovIoAchAddenda99Contested {
    fn contested_return_code_field(&self) -> String {
        string_field(&self.contested_return_code, 3)
    }
}

fn string_field(s: &str, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let m = max - ln;
        MOOV_IO_ACH_STRINGZEROS.get(&(m as usize)).cloned().unwrap_or_else(|| "0".repeat(m as usize)) + s
    }
}

fn populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut map = HashMap::with_capacity(max);
    for i in 0..max {
        map.insert(i, "0".repeat(i));
    }
    map
}

struct MoovIoAchConverters;

