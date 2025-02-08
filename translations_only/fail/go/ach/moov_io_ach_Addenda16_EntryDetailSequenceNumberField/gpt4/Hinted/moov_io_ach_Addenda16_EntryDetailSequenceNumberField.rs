
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

struct MoovIoAchAddenda16 {
    entry_detail_sequence_number: i32,
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: usize) -> String {
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

impl MoovIoAchAddenda16 {
    fn entry_detail_sequence_number_field(&self) -> String {
        let converters = MoovIoAchConverters;
        converters.numeric_field(self.entry_detail_sequence_number, 7)
    }
}

fn main() {
    let addenda16 = MoovIoAchAddenda16 {
        entry_detail_sequence_number: 235539156,
    };
    println!("{}", addenda16.entry_detail_sequence_number_field());
}
