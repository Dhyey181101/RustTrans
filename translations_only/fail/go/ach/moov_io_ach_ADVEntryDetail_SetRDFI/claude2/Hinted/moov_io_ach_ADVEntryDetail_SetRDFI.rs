
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0"));

struct MoovIoAchConverters;

struct MoovIoAchAdvEntryDetail {
    rdfi_identification: String,
    check_digit: String,
}

impl MoovIoAchAdvEntryDetail {
    fn set_rdfi(&mut self, rdfi: &str) -> &mut Self {
        let s = MoovIoAchConverters::string_field(rdfi, 9);
        self.rdfi_identification = MoovIoAchConverters::parse_string_field(&s[..8]);
        self.check_digit = MoovIoAchConverters::parse_string_field(&s[8..9]);
        self
    }
}

impl MoovIoAchConverters {
    fn string_field(s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = max - ln;
            let pad = MOOV_IO_ACH_STRINGZEROS.get(&(m as usize)).unwrap();
            format!("{}{}", pad, s)
        }
    }

    fn parse_string_field(s: &str) -> String {
        s.trim().to_string()
    }
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i as usize, zero.repeat(i as usize));
    }
    out
}

