
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

struct MoovIoAchIATEntryDetail {
    addenda_records: i32,
}

impl MoovIoAchIATEntryDetail {
    fn addenda_records_field(&self) -> String {
        MoovIoAchConverters::numeric_field(self.addenda_records, 4)
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
    println!("{}", MoovIoAchIATEntryDetail { addenda_records: -6000129 }.addenda_records_field());
    println!("{}", MoovIoAchIATEntryDetail { addenda_records: 10032 }.addenda_records_field());
    println!("{}", MoovIoAchIATEntryDetail { addenda_records: 168 }.addenda_records_field());
    println!("{}", MoovIoAchIATEntryDetail { addenda_records: 1288 }.addenda_records_field());
}
