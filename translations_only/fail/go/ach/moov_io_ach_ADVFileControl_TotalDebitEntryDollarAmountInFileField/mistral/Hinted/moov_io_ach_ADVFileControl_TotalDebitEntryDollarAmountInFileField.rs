

use std::collections::HashMap;
use std::fmt;
use std::string::String;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = MoovIoAchFileControl::string_zeros(m as usize);
            return pad + &s;
        }
    }
}

struct MoovIoAchFileControl {
    id: String,
    batch_count: i32,
    block_count: i32,
    entry_addenda_count: i32,
    entry_hash: i32,
    total_debit_entry_dollar_amount_in_file: i32,
    total_credit_entry_dollar_amount_in_file: i32,
}

impl MoovIoAchFileControl {
    fn total_debit_entry_dollar_amount_in_file_field(&self) -> String {
        let converters = Box::new(MoovIoAchConverters);
        converters.numeric_field(self.total_debit_entry_dollar_amount_in_file, 20)
    }

    fn string_zeros(n: usize) -> String {
        let mut out = HashMap::new();
        for i in 0..=n {
            out.insert(i, "0".repeat(i));
        }
        out[&n].clone()
    }
}

impl fmt::Display for MoovIoAchFileControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: {}, BatchCount: {}, BlockCount: {}, EntryAddendaCount: {}, EntryHash: {}, TotalDebitEntryDollarAmountInFile: {}, TotalCreditEntryDollarAmountInFile: {}",
            self.id, self.batch_count, self.block_count, self.entry_addenda_count, self.entry_hash, self.total_debit_entry_dollar_amount_in_file, self.total_credit_entry_dollar_amount_in_file
        )
    }
}

fn main() {
    let mut fc = MoovIoAchFileControl {
        id: "test_id".to_string(),
        batch_count: 5,
        block_count: 100,
        entry_addenda_count: 25,
        entry_hash: 12345,
        total_debit_entry_dollar_amount_in_file: 1000,
        total_credit_entry_dollar_amount_in_file: 2000,
    };

    println!("{}", fc.total_debit_entry_dollar_amount_in_file_field());
}

