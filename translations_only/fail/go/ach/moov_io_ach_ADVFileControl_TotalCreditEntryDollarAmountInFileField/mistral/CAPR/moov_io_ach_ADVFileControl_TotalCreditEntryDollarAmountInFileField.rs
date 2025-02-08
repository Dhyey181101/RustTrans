

use std::collections::HashMap;
use std::fmt;

const ZEROS: &str = "0";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max {
            return s[l - max..].to_string();
        }
        let m = max - l;
        let pad = MoovIoAchFileControl::get_zeros(m);
        pad + &s
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
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchFileControl {
    fn total_credit_entry_dollar_amount_in_file_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.total_credit_entry_dollar_amount_in_file, 20)
    }

    fn get_zeros(n: usize) -> String {
        let mut out = HashMap::new();
        for i in 0..=n {
            out.insert(i, ZEROS.repeat(i));
        }
        out[&n].to_string()
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
    let file_control = MoovIoAchFileControl {
        id: "12345".to_string(),
        batch_count: 5,
        block_count: 50,
        entry_addenda_count: 10,
        entry_hash: 20,
        total_debit_entry_dollar_amount_in_file: 1000,
        total_credit_entry_dollar_amount_in_file: 2000,
        moov_io_ach_converters: Box::new(MoovIoAchConverters),
    };
    println!("{}", file_control);
}

