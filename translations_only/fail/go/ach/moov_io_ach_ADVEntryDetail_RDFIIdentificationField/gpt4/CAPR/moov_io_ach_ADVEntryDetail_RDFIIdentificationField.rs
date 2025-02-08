
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<Box<HashMap<usize, String>>> = None;

struct MoovIoAchADVEntryDetail {
    rdfi_identification: String,
}

struct MoovIoAchConverters;

impl MoovIoAchADVEntryDetail {
    fn rdfi_identification_field(&self) -> String {
        MoovIoAchConverters::string_field(&self.rdfi_identification, 8)
    }
}

impl MoovIoAchConverters {
    fn string_field(s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s.chars().take(max).collect();
        }

        let m = max - ln;
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
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..=max {
        out.insert(i, zero.repeat(i));
    }
    out
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(Box::new(moov_io_ach_populate_map(94, "0")));
    }
}
