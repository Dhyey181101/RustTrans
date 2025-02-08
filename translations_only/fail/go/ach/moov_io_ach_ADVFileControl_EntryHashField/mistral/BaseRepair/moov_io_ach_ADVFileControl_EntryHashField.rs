

use std::collections::HashMap;
use std::fmt;
use std::str;

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
    id: String,
    batch_count: i32,
    block_count: i32,
    entry_addenda_count: i32,
    entry_hash: i32,
    total_debit_entry_dollar_amount_in_file: f64,
    total_credit_entry_dollar_amount_in_file: f64,
}

impl MoovIoAchAdvFileControl {
    fn entry_hash_field(&self) -> String {
        MoovIoAchConverters {}.numeric_field(self.entry_hash, 10)
    }
}

impl fmt::Display for MoovIoAchAdvFileControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: {}, BatchCount: {}, BlockCount: {}, EntryAddendaCount: {}, EntryHash: {}, TotalDebitEntryDollarAmountInFile: {}, TotalCreditEntryDollarAmountInFile: {}",
            self.id, self.batch_count, self.block_count, self.entry_addenda_count, self.entry_hash, self.total_debit_entry_dollar_amount_in_file, self.total_credit_entry_dollar_amount_in_file
        )
    }
}

fn main() {
    let a = Box::new(MoovIoAchAdvFileControl {
        id: "123".to_string(),
        batch_count: 10,
        block_count: 20,
        entry_addenda_count: 30,
        entry_hash: 40,
        total_debit_entry_dollar_amount_in_file: 50.0,
        total_credit_entry_dollar_amount_in_file: 60.0,
    });
    println!("{}", a.entry_hash_field());
}

