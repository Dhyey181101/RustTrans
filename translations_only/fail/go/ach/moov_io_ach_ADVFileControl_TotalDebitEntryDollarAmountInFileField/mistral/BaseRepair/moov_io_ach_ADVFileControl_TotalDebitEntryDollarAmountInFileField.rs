

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &str = "0";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = MoovIoAchFileControl::get_zeros(m as usize);
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
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchFileControl {
    fn total_debit_entry_dollar_amount_in_file_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.total_debit_entry_dollar_amount_in_file, 20)
    }

    fn get_zeros(n: usize) -> String {
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
    let moov_io_ach_file_control = MoovIoAchFileControl {
        id: "test_id".to_string(),
        batch_count: 10,
        block_count: 100,
        entry_addenda_count: 200,
        entry_hash: 300,
        total_debit_entry_dollar_amount_in_file: 400,
        total_credit_entry_dollar_amount_in_file: 500,
        moov_io_ach_converters: Box::new(MoovIoAchConverters),
    };

    println!("{}", moov_io_ach_file_control.total_debit_entry_dollar_amount_in_file_field());
}

