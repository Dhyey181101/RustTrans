
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

struct MoovIoAchAddenda15 {
    entry_detail_sequence_number: i32,
}

struct MoovIoAchConverters;

impl MoovIoAchAddenda15 {
    fn entry_detail_sequence_number_field(&self) -> String {
        MoovIoAchConverters::numeric_field(self.entry_detail_sequence_number, 7)
    }
}

impl MoovIoAchConverters {
    fn numeric_field(n: i32, max: usize) -> String {
        let s = n.abs().to_string();
        if s.len() > max {
            s[s.len() - max..].to_string()
        } else {
            let m = max - s.len();
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                format!("{}{}", pad, s)
            } else {
                // slow path
                "0".repeat(m) + &s
            }
        }
    }
}

fn main() {
    // Example usage
    let addenda15 = MoovIoAchAddenda15 {
        entry_detail_sequence_number: 13386233,
    };
    println!("{}", addenda15.entry_detail_sequence_number_field());
}
