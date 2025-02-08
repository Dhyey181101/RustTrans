
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

struct MoovIoAchADVBatchControl {
    total_credit_entry_dollar_amount: i32,
}

impl MoovIoAchADVBatchControl {
    fn total_credit_entry_dollar_amount_field(&self) -> String {
        self.numeric_field(self.total_credit_entry_dollar_amount, 20)
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

#[macro_use]
extern crate lazy_static;

fn main() {
    let control = MoovIoAchADVBatchControl {
        total_credit_entry_dollar_amount: -150929419,
    };
    println!("{}", control.total_credit_entry_dollar_amount_field());

    let control2 = MoovIoAchADVBatchControl {
        total_credit_entry_dollar_amount: -66313,
    };
    println!("{}", control2.total_credit_entry_dollar_amount_field());
}
