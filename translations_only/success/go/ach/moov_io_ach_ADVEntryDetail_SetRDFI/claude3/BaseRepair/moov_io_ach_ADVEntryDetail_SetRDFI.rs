

use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

fn moov_io_ach_string_field(s: &str, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        return s.chars().take(max as usize).collect();
    }

    let m = (max - ln) as usize;
    unsafe {
        if let Some(ref map) = MOOV_IO_ACH_STRING_ZEROS {
            if let Some(pad) = map.get(&m) {
                return pad.clone() + s;
            }
        } else {
            MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
        }
    }

    "0".repeat(m) + s
}

fn moov_io_ach_parse_string_field(r: &str) -> String {
    r.trim().to_string()
}

struct MoovIoAchAdventryDetail {
    rdfi_identification: String,
    check_digit: String,
}

impl MoovIoAchAdventryDetail {
    fn set_rdfi(&mut self, rdfi: &str) -> Box<MoovIoAchAdventryDetail> {
        let s = moov_io_ach_string_field(rdfi, 9);
        self.rdfi_identification = moov_io_ach_parse_string_field(&s[..8]);
        self.check_digit = moov_io_ach_parse_string_field(&s[8..9]);
        Box::new(MoovIoAchAdventryDetail {
            rdfi_identification: self.rdfi_identification.clone(),
            check_digit: self.check_digit.clone(),
        })
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: u32) -> String {
        moov_io_ach_string_field(s, max)
    }

    fn parse_string_field(&self, r: &str) -> String {
        moov_io_ach_parse_string_field(r)
    }
}

