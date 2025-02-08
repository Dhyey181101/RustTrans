


use std::fmt;
use std::collections::HashMap;
use std::convert::TryInto;
use std::str;

const ZEROS: &str = "0";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s[(l - max) as usize..].to_string()
        } else {
            let m = max - l;
            let pad = get_zeros(m as usize);
            format!("{}{}", pad, s)
        }
    }
}

fn get_zeros(n: usize) -> String {
    let mut out = String::new();
    for _ in 0..n {
        out.push_str(ZEROS);
    }
    out
}

struct MoovIoAchAdvFileControl {
    entry_addenda_count: i32,
    converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAdvFileControl {
    fn entry_addenda_count_field(&self) -> String {
        self.converters.numeric_field(self.entry_addenda_count, 8)
    }
}

impl fmt::Display for MoovIoAchAdvFileControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: ?, BatchCount: ?, BlockCount: ?, EntryAddendaCount: {}, EntryHash: ?, TotalDebitEntryDollarAmountInFile: ?, TotalCreditEntryDollarAmountInFile: ?, validator: ?",
            self.entry_addenda_count
        )
    }
}

fn main() {
    let mut map = HashMap::new();
    for i in 0..94 {
        let zeros = get_zeros(i);
        map.insert(i, zeros);
    }
    let converters = Box::new(MoovIoAchConverters);
    let fc = MoovIoAchAdvFileControl { entry_addenda_count: 123, converters };
    println!("{}", fc.entry_addenda_count_field());
}


