
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<i32, String>> = None;

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

struct MoovIoAchEntryDetail {
    rdfi_identification: String,
    check_digit: String,
    converters: MoovIoAchConverters,
}

impl MoovIoAchEntryDetail {
    fn set_rdfi(&mut self, rdfi: String) -> Box<MoovIoAchEntryDetail> {
        let s = self.converters.string_field(&rdfi, 9);
        self.rdfi_identification = self.converters.parse_string_field(&s[..8]);
        self.check_digit = self.converters.parse_string_field(&s[8..9]);
        Box::new(MoovIoAchEntryDetail {
            rdfi_identification: self.rdfi_identification.clone(),
            check_digit: self.check_digit.clone(),
            converters: self.converters.clone(),
        })
    }
}

#[derive(Clone)]
struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s.chars().take(max as usize).collect();
        }

        let m = (max - ln) as i32;
        let pad = unsafe {
            MOOV_IO_ACH_STRING_ZEROS
                .as_ref()
                .unwrap()
                .get(&m)
                .unwrap()
                .clone()
        };
        format!("{}{}", pad, s)
    }

    fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
    }
}
