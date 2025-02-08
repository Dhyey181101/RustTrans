
use std::collections::HashMap;

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<i32, String> = moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

struct MoovIoAchADVFileControl {
    total_debit_entry_dollar_amount_in_file: i32,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchADVFileControl {
    fn total_debit_entry_dollar_amount_in_file_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.total_debit_entry_dollar_amount_in_file, 20)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s.chars().skip((l - max) as usize).collect()
        } else {
            let m = (max - l) as i32;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                format!("{}{}", pad, s)
            } else {
                // slow path
                "0".repeat(m as usize) + &s
            }
        }
    }
}

#[macro_use]
extern crate lazy_static;

fn main() {}
