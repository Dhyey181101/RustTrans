
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

struct MoovIoAchBatchControl {
    total_credit_entry_dollar_amount: i32,
}

impl MoovIoAchBatchControl {
    fn total_credit_entry_dollar_amount_field(&self) -> String {
        MoovIoAchConverters::numeric_field(self.total_credit_entry_dollar_amount, 12)
    }

    fn new(total_credit_entry_dollar_amount: i32) -> Box<Self> {
        Box::new(Self {
            total_credit_entry_dollar_amount,
        })
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
    let control = MoovIoAchBatchControl::new(127336599);
    println!("{}", control.total_credit_entry_dollar_amount_field());

    let control_negative = MoovIoAchBatchControl::new(-294913);
    println!("{}", control_negative.total_credit_entry_dollar_amount_field());
}
