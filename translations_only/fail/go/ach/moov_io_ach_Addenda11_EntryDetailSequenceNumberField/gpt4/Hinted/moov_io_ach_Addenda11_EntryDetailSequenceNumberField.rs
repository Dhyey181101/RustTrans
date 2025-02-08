
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

struct MoovIoAchAddenda11 {
    entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda11 {
    fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }

    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max {
            s[l - max..].to_string()
        } else {
            let m = max - l;
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
    let addenda11 = MoovIoAchAddenda11 {
        entry_detail_sequence_number: 33554432,
    };
    println!("{}", addenda11.entry_detail_sequence_number_field());
}
