
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

struct MoovIoAchADVEntryDetail {
    sequence_number: i32,
}

impl MoovIoAchADVEntryDetail {
    fn sequence_number_field(&self) -> String {
        MoovIoAchConverters::numeric_field(self.sequence_number, 4)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(n: i32, max: usize) -> String {
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
    println!("{}", MoovIoAchADVEntryDetail { sequence_number: 168447233 }.sequence_number_field());
    println!("{}", MoovIoAchADVEntryDetail { sequence_number: -162320128 }.sequence_number_field());
    println!("{}", MoovIoAchADVEntryDetail { sequence_number: -684 }.sequence_number_field());
    println!("{}", MoovIoAchADVEntryDetail { sequence_number: 505 }.sequence_number_field());
}
