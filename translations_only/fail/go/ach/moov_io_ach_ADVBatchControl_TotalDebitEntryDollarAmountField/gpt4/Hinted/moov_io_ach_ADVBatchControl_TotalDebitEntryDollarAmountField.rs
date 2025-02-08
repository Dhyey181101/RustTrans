
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

struct MoovIoAchADVBatchControl {
    total_debit_entry_dollar_amount: i32,
}

impl MoovIoAchADVBatchControl {
    fn total_debit_entry_dollar_amount_field(&self) -> String {
        self.numeric_field(self.total_debit_entry_dollar_amount, 20)
    }

    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
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
    let control = MoovIoAchADVBatchControl {
        total_debit_entry_dollar_amount: 6571038,
    };
    println!("{}", control.total_debit_entry_dollar_amount_field());

    let control = MoovIoAchADVBatchControl {
        total_debit_entry_dollar_amount: 168886304,
    };
    println!("{}", control.total_debit_entry_dollar_amount_field());
}
