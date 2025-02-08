
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

struct MoovIoAchIATBatchHeader {
    odfi_identification: String,
}

struct MoovIoAchConverters;

impl MoovIoAchIATBatchHeader {
    fn odfi_identification_field(&self) -> String {
        MoovIoAchConverters::string_field(&self.odfi_identification, 8)
    }
}

impl MoovIoAchConverters {
    fn string_field(s: &str, max: usize) -> String {
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
}

fn main() {
    let example0 = MoovIoAchIATBatchHeader {
        odfi_identification: String::from(",\"\"1"),
    };
    println!("{}", example0.odfi_identification_field());

    let example1 = MoovIoAchIATBatchHeader {
        odfi_identification: String::from(",>\"0"),
    };
    println!("{}", example1.odfi_identification_field());
}
