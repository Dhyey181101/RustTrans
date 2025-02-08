
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchBatchHeader {
    odfi_identification: String,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchBatchHeader {
    fn odfi_identification_field(&self) -> String {
        self.moov_io_ach_converters.string_field(&self.odfi_identification, 8)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s.chars().take(max).collect::<String>();
        }

        let m = max - ln;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            return format!("{}{}", pad, s);
        }

        // slow path
        format!("{}{}", "0".repeat(m), s)
    }
}

fn main() {}
