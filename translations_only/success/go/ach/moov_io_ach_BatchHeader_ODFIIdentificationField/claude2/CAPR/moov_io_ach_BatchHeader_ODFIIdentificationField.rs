
use std::collections::HashMap;
use std::str;

use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

struct BatchHeader {
    odfi_identification: String,
}

impl BatchHeader {
    fn odfi_identification_field(&self) -> String {
        Converters::string_field(&self.odfi_identification, 8)
    }
}

struct Converters;

impl Converters {
    fn string_field(s: &str, max: u32) -> String {
        let len = s.chars().count() as u32;
        if len > max {
            s[..max as usize].to_string()
        } else {
            let missing = max - len;
            let pad = MOOV_IO_ACH_STRINGZEROS.get(&(missing as usize)).cloned().unwrap_or_else(|| "0".repeat(missing as usize));
            format!("{}{}", pad, s)
        }
    }
}

fn populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut map = HashMap::with_capacity(max);
    for i in 0..max {
        map.insert(i, zero.repeat(i));
    }
    map
}
