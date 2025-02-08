

use std::collections::HashMap;
use std::str;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

fn moov_io_ach_string_field(s: &str, max: u32) -> String {
    let ln = s.chars().count();
    if ln > max as usize {
        return s[..max as usize].to_string();
    }

    let m = (max as usize) - ln;
    unsafe {
        if MOOV_IO_ACH_STRING_ZEROS.is_none() {
            MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
        }
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap().get(&m) {
            return pad.to_string() + s;
        }
    }

    "0".repeat(m) + s
}

fn moov_io_ach_parse_string_field(r: &str) -> String {
    r.trim().to_string()
}

struct MoovIoAchEntryDetail {
    rdfi_identification: String,
    check_digit: String,
}

impl MoovIoAchEntryDetail {
    fn set_rdfi(&mut self, rdfi: &str) -> Box<MoovIoAchEntryDetail> {
        let s = moov_io_ach_string_field(rdfi, 9);
        self.rdfi_identification = moov_io_ach_parse_string_field(&s[..8]);
        self.check_digit = moov_io_ach_parse_string_field(&s[8..9]);
        Box::new(MoovIoAchEntryDetail {
            rdfi_identification: self.rdfi_identification.clone(),
            check_digit: self.check_digit.clone(),
        })
    }
}

