

use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    let mut map = HashMap::with_capacity(94);
    for i in 0..94 {
        map.insert(i, "0".repeat(i));
    }
    map
});

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
            let mut pad = MOOV_IO_ACH_STRINGZEROS.get(&(max as usize - len as usize)).cloned().unwrap_or_default();
            pad.push_str(s);
            pad
        }
    }
}

