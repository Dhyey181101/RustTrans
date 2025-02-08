

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &str = "0";

struct MoovIoAchFileControl {
    total_debit: i32,
    converters: MoovIoAchConverters,
}

struct MoovIoAchConverters;

impl MoovIoAchFileControl {
    fn total_debit_entry_dollar_amount_in_file_field(&self) -> String {
        self.numeric_field(self.total_debit, 12)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            let start_index = max - (l - max);
            s[start_index as usize..].to_string()
        } else {
            let m = max - l;
            let pad = MoovIoAchFileControl::get_pad(m);
            format!("{}{}", pad, s)
        }
    }

    fn get_pad(n: u32) -> String {
        let mut out = HashMap::new();
        for i in 0..=n {
            out.insert(i, "0".repeat(i as usize));
        }
        out.get(&n).unwrap_or(&ZEROS.repeat(n as usize)).to_string()
    }
}

impl fmt::Display for MoovIoAchFileControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "total_debit: {}, converters: {{}}",
            self.total_debit
        )
    }
}

fn main() {
    let fc = MoovIoAchFileControl {
        total_debit: 12345,
        converters: MoovIoAchConverters {},
    };
    println!("{}", fc.total_debit_entry_dollar_amount_in_file_field());
}

