

use std::collections::HashMap;
use std::fmt;
use std::str;

const SERVICE_CLASS_CODE_MIXED: &str = "200";
const SERVICE_CLASS_CODE_CREDITS: &str = "220";
const SERVICE_CLASS_CODE_DEBITS: &str = "225";

struct MoovIoAchBatchControl {
    total_debit_entry_dollar_amount: i32,
    // ... other fields elided ...
    converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters {
    // ... other fields elided ...
}

impl MoovIoAchBatchControl {
    fn total_debit_entry_dollar_amount_field(&self) -> String {
        self.converters.numeric_field(self.total_debit_entry_dollar_amount, 12)
    }
}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s[(l - max) as usize..].to_string()
        } else {
            let m = max - l;
            let pad = get_zeros(m as usize);
            pad + &s
        }
    }
}

fn get_zeros(n: usize) -> String {
    let mut out = String::new();
    for _ in 0..n {
        out.push('0');
    }
    out
}

impl fmt::Display for MoovIoAchBatchControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "total_debit_entry_dollar_amount: {}" ,
            self.total_debit_entry_dollar_amount
        )
    }
}

fn main() {
    let batch_control = MoovIoAchBatchControl {
        total_debit_entry_dollar_amount: 123,
        converters: Box::new(MoovIoAchConverters {}),
    };
    println!("{}", batch_control.total_debit_entry_dollar_amount_field());
}

