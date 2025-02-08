
use std::fmt;
use std::collections::HashMap;
use std::str;
use std::iter;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max {
            s[l-max..].to_string()
        } else {
            let m = max - l;
            let pad = get_pad_string(m);
            pad + &s
        }
    }
}

fn get_pad_string(n: usize) -> String {
    iter::repeat("0").take(n).collect::<String>()
}

struct MoovIoAchAdvfilecontrol {
    entry_hash: i32,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAdvfilecontrol {
    fn entry_hash_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.entry_hash, 10)
    }
}

struct Entry {
    id: String,
    batch_count: i32,
    block_count: i32,
    entry_addenda_count: i32,
    entry_hash: i32,
    total_debit_entry_dollar_amount_in_file: f64,
    total_credit_entry_dollar_amount_in_file: f64,
}

impl Entry {
    fn new() -> Entry {
        Entry {
            id: String::from(""),
            batch_count: 0,
            block_count: 0,
            entry_addenda_count: 0,
            entry_hash: 0,
            total_debit_entry_dollar_amount_in_file: 0.0,
            total_credit_entry_dollar_amount_in_file: 0.0,
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID: {}, BatchCount: {}, BlockCount: {}, EntryAddendaCount: {}, EntryHash: {}, TotalDebitEntryDollarAmountInFile: {}, TotalCreditEntryDollarAmountInFile: {}",
            self.id, self.batch_count, self.block_count, self.entry_addenda_count, self.entry_hash, self.total_debit_entry_dollar_amount_in_file, self.total_credit_entry_dollar_amount_in_file)
    }
}

fn main() {
    let mut entry = Entry::new();
    entry.id = String::from("1");
    entry.batch_count = 5;
    entry.block_count = 50;
    entry.entry_addenda_count = 15;
    entry.entry_hash = 123456;
    entry.total_debit_entry_dollar_amount_in_file = 123456.0;
    entry.total_credit_entry_dollar_amount_in_file = 654321.0;
    println!("{}", entry);
    let mut adv_file_control = MoovIoAchAdvfilecontrol {
        entry_hash: 123456,
        moov_io_ach_converters: MoovIoAchConverters,
    };
    println!("Entry Hash Field: {}", adv_file_control.entry_hash_field());
}
