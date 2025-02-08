
use std::collections::HashMap;
use std::string::String;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    moov_io_ach_populate_map(94, "0".to_string())
});

struct MoovIoAchEntryDetail {
    rdfi_identification: String,
    check_digit: String,
}

impl MoovIoAchEntryDetail {
    fn set_rdfi(&mut self, rdfi: String) -> &mut Self {
        let s = MoovIoAchConverters::string_field(&rdfi, 9);
        self.rdfi_identification = MoovIoAchConverters::parse_string_field(&s[..8]);
        self.check_digit = MoovIoAchConverters::parse_string_field(&s[8..9]);
        self
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = max - ln;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&(m as usize)) {
                format!("{}{}", pad, s)
            } else {
                format!("{}{}", "0".repeat(m as usize), s)
            }
        }
    }

    fn parse_string_field(s: &str) -> String {
        s.trim().to_string()
    }
}

fn moov_io_ach_populate_map(max: i32, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i as usize, zero.repeat(i as usize));
    }
    out
}

