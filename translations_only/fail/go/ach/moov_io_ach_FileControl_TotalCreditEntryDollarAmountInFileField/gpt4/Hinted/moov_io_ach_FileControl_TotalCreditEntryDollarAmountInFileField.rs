
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

struct MoovIoAchFileControl {
    total_credit_entry_dollar_amount_in_file: i32,
}

impl MoovIoAchFileControl {
    fn total_credit_entry_dollar_amount_in_file_field(&self) -> String {
        self.numeric_field(self.total_credit_entry_dollar_amount_in_file, 12)
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
    let control = MoovIoAchFileControl {
        total_credit_entry_dollar_amount_in_file: 5736970,
    };
    println!("{}", control.total_credit_entry_dollar_amount_in_file_field());

    let control2 = MoovIoAchFileControl {
        total_credit_entry_dollar_amount_in_file: 468749094,
    };
    println!("{}", control2.total_credit_entry_dollar_amount_in_file_field());
}
