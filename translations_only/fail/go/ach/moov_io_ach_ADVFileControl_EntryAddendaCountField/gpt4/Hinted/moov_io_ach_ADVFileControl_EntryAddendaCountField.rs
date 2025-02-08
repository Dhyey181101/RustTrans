
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

struct MoovIoAchADVFileControl {
    entry_addenda_count: i32,
}

impl MoovIoAchADVFileControl {
    fn entry_addenda_count_field(&self) -> String {
        MoovIoAchConverters::numeric_field(self.entry_addenda_count, 8)
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
    // Example usage
    let control = MoovIoAchADVFileControl {
        entry_addenda_count: 153814142,
    };
    println!("{}", control.entry_addenda_count_field());
}
