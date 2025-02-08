
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(0, "0".to_string());
    map
});

struct MoovIoAchIatEntryDetail {
    rdfi_identification: String,
}

impl MoovIoAchIatEntryDetail {
    fn rdfi_identification_field(&self) -> String {
        MoovIoAchConverters::string_field(self.rdfi_identification.clone(), 8)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(s: String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = max - ln;
            let pad = "0".repeat(m as usize);
            format!("{}{}", pad, s)
        }
    }
}

