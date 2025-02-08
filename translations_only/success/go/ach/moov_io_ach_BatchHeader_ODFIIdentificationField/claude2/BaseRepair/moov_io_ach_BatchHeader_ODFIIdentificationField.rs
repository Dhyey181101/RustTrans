
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| HashMap::new());

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
        let len = s.len() as u32;
        if len > max {
            s[..max as usize].to_string()
        } else {
            let missing = max - len;
            let pad = "0".repeat(missing as usize);
            format!("{}{}", pad, s)
        }
    }
}
