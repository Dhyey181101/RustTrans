

use std::collections::HashMap;
use std::fmt;

const ZEROS: &str = "0";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max {
            s[l - max..].to_string()
        } else {
            let m = max - l;
            let pad = get_pad(m);
            pad.chars().chain(s.chars()).collect()
        }
    }
}

fn get_pad(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, "0".repeat(i));
    }
    out[&n].clone()
}

struct MoovIoAchAdvFileControl {
    total_debit: i32,
    converters: MoovIoAchConverters,
}

impl MoovIoAchAdvFileControl {
    fn total_debit_entry_dollar_amount_in_file_field(&self) -> String {
        self.converters.numeric_field(self.total_debit, 20)
    }
}

impl fmt::Display for MoovIoAchAdvFileControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: ? BatchCount: ? BlockCount: ? EntryAddendaCount: ? EntryHash: ? TotalDebitEntryDollarAmountInFile: {} TotalCreditEntryDollarAmountInFile: ?",
            self.total_debit
        )
    }
}

fn main() {
    let a = MoovIoAchAdvFileControl {
        total_debit: 123456,
        converters: MoovIoAchConverters,
    };
    println!("{}", a);
}

