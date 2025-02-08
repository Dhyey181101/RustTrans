
use std::collections::HashMap;
use std::str;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for i in 0..94 {
        map.insert(i, "0".repeat(i));
    }
    map
});

struct MoovIoAchAddenda99 {
    original_dfi: String,
}

impl MoovIoAchAddenda99 {
    fn original_dfi_field(&self) -> String {
        Converters::string_field(&self.original_dfi, 8)
    }
}

struct Converters;

impl Converters {
    fn string_field(s: &String, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = max - ln;
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&(m as usize)).unwrap().clone();
            format!("{}{}", pad, s)
        }
    }
}
