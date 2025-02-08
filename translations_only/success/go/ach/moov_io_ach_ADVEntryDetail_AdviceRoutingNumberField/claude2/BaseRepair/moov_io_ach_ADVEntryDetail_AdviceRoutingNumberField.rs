

use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| HashMap::new());

struct MoovIoAchAdvEntryDetail {
    advice_routing_number: String,
    moov_io_ach_converters: MoovIoAchConverters,
}

struct MoovIoAchConverters;

impl MoovIoAchAdvEntryDetail {
    fn advice_routing_number_field(&self) -> String {
        self.moov_io_ach_converters.string_field(&self.advice_routing_number, 9)
    }
}

impl MoovIoAchConverters {
    fn string_field(&self, s: &String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = (max - ln) as usize;
            let pad = "0".repeat(m);
            format!("{}{}", pad, s)
        }
    }
}

