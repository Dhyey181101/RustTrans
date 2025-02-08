
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

struct MoovIoAchAdventryDetail {
    rdfi_identification: String,
    check_digit: String,
}

impl MoovIoAchAdventryDetail {
    fn set_rdfi(&mut self, rdfi: String) -> &mut Self {
        let s = moov_io_ach_string_field(rdfi, 9);
        self.rdfi_identification = moov_io_ach_parse_string_field(&s[..8]);
        self.check_digit = moov_io_ach_parse_string_field(&s[8..9]);
        self
    }
}

fn moov_io_ach_string_field(s: String, max: usize) -> String {
    let ln = s.chars().count();
    if ln > max {
        return s[..max].to_string();
    }

    let m = max - ln;
    if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
        return pad.to_string() + &s;
    }

    "0".repeat(m) + &s
}

fn moov_io_ach_parse_string_field(r: &str) -> String {
    r.trim().to_string()
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}
