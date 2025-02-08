
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

#[derive(Clone)]
struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s.chars().take(max).collect()
        } else {
            let m = max - ln;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                format!("{}{}", pad, s)
            } else {
                "0".repeat(m) + s
            }
        }
    }

    fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }
}

struct MoovIoAchADVEntryDetail {
    rdfi_identification: String,
    check_digit: String,
    converters: Option<Box<MoovIoAchConverters>>,
}

impl MoovIoAchADVEntryDetail {
    fn new() -> Self {
        MoovIoAchADVEntryDetail {
            rdfi_identification: String::new(),
            check_digit: String::new(),
            converters: Some(Box::new(MoovIoAchConverters)),
        }
    }

    fn set_rdfi(&mut self, rdfi: &str) -> &mut Self {
        let converters = self.converters.get_or_insert_with(|| Box::new(MoovIoAchConverters));
        let s = converters.string_field(rdfi, 9);
        let (rdfi_identification, check_digit) = s.split_at(8);
        self.rdfi_identification = converters.parse_string_field(rdfi_identification);
        self.check_digit = converters.parse_string_field(check_digit);
        self
    }
}

fn main() {}
