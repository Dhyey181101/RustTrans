
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

struct MoovIoAchAddenda99Dishonored {
    original_receiving_dfi_identification: String,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda99Dishonored {
    fn original_receiving_dfi_identification_field(&self) -> String {
        self.moov_io_ach_converters.string_field(&self.original_receiving_dfi_identification, 8)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s.chars().take(max).collect();
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

#[macro_use]
extern crate lazy_static;

fn main() {
    // Example usage
    let converter = Box::new(MoovIoAchConverters {});
    let addenda99 = MoovIoAchAddenda99Dishonored {
        original_receiving_dfi_identification: "12345678".to_string(),
        moov_io_ach_converters: converter,
    };
    println!("{}", addenda99.original_receiving_dfi_identification_field());
}
