
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

struct MoovIoAchAddenda10 {
    foreign_payment_amount: i32,
}

impl MoovIoAchAddenda10 {
    fn foreign_payment_amount_field(&self) -> String {
        self.numeric_field(self.foreign_payment_amount, 18)
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
    let addenda10 = MoovIoAchAddenda10 {
        foreign_payment_amount: 524388,
    };
    println!("{}", addenda10.foreign_payment_amount_field());

    let addenda10_negative = MoovIoAchAddenda10 {
        foreign_payment_amount: -744389127,
    };
    println!("{}", addenda10_negative.foreign_payment_amount_field());
}
