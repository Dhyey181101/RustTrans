

use std::collections::HashMap;
use std::fmt;
use std::str;

#[derive(Debug)]
struct MoovIoAchFileControl {
    total_debit: i32,
    converters: Box<MoovIoAchConverters>,
}

#[derive(Debug)]
struct MoovIoAchConverters;

impl MoovIoAchFileControl {
    fn total_debit_entry_dollar_amount_in_file_field(&self) -> String {
        self.converters.numeric_field(self.total_debit, 12)
    }
}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = get_pad_string(m as usize);
            return pad.repeat(m as usize) + &s;
        }
    }
}

fn get_pad_string(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, "0".repeat(i));
    }
    return out[&n].clone();
}

impl fmt::Display for MoovIoAchFileControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "total_debit: {}, converters: {:?}",
            self.total_debit, *self.converters
        )
    }
}

fn main() {
    let fc = MoovIoAchFileControl {
        total_debit: 123456,
        converters: Box::new(MoovIoAchConverters {}),
    };
    println!("{}", fc.total_debit_entry_dollar_amount_in_file_field());
}

