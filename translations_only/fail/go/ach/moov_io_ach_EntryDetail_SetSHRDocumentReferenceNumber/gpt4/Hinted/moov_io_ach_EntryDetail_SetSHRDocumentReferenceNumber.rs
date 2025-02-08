
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

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

struct MoovIoAchEntryDetail {
    identification_number: String,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchEntryDetail {
    fn set_shr_document_reference_number(&mut self, s: &str) {
        self.identification_number = self.identification_number.clone() + &self.moov_io_ach_converters.string_field(s, 11);
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s.chars().take(max).collect::<String>();
        }

        // Pad with preallocated string
        let m = max - ln;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            return pad.clone() + s;
        }
        // slow path
        "0".repeat(m) + s
    }
}

fn main() {
    // Example usage
    let mut entry_detail = MoovIoAchEntryDetail {
        identification_number: String::new(),
        moov_io_ach_converters: Box::new(MoovIoAchConverters),
    };
    entry_detail.set_shr_document_reference_number("@@-X@");
    println!("{}", entry_detail.identification_number);
}
