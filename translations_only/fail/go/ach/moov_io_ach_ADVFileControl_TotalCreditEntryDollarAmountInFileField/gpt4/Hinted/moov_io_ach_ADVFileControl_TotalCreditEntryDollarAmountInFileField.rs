
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

struct MoovIoAchADVFileControl {
    total_credit_entry_dollar_amount_in_file: i32,
}

impl MoovIoAchADVFileControl {
    fn total_credit_entry_dollar_amount_in_file_field(&self) -> String {
        numeric_field(self.total_credit_entry_dollar_amount_in_file, 20)
    }
}

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

#[macro_use]
extern crate lazy_static;

fn main() {
    let control = MoovIoAchADVFileControl {
        total_credit_entry_dollar_amount_in_file: 150326506,
    };
    println!("{}", control.total_credit_entry_dollar_amount_in_file_field());

    let control2 = MoovIoAchADVFileControl {
        total_credit_entry_dollar_amount_in_file: 1310466,
    };
    println!("{}", control2.total_credit_entry_dollar_amount_in_file_field());
}
