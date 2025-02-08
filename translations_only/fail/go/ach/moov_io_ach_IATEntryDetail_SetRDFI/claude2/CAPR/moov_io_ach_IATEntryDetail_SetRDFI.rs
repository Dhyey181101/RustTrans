
use std::collections::HashMap;
use std::string::String;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0"));

struct MoovIoAchIatEntryDetail {
    rdfi_identification: String,
    check_digit: String,
}

impl MoovIoAchIatEntryDetail {
    fn set_rdfi(&mut self, rdfi: &str, converters: &MoovIoAchConverters) -> &mut Self {
        let s = converters.string_field(rdfi, 9);
        self.rdfi_identification = converters.parse_string_field(&s[..8]);
        self.check_digit = converters.parse_string_field(&s[8..9]);
        self
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = max - ln;
            let pad = MOOV_IO_ACH_STRINGZEROS.get(&(m as usize)).unwrap();
            format!("{}{}", pad, s)
        }
    }

    fn parse_string_field(&self, s: &str) -> String {
        s.trim().to_string()
    }
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

