

use std::collections::HashMap;
use std::fmt;

const ZEROS: &str = "0";

struct MoovIoAchFileControl {
    total_debit: i32,
    converters: MoovIoAchConverters,
}

impl MoovIoAchFileControl {
    fn total_debit_entry_dollar_amount_in_file_field(&self) -> String {
        self.converters.numeric_field(self.total_debit, 12)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s[(l-max) as usize..].to_string()
        } else {
            let m = max - l;
            let pad = get_pad(m as usize);
            pad + &s
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

struct MoovIoAchStringZeros {
    m: HashMap<usize, String>,
}

impl MoovIoAchStringZeros {
    fn new(max: usize, zero: &str) -> MoovIoAchStringZeros {
        let mut m = HashMap::new();
        for i in 0..=max {
            m.insert(i, zero.repeat(i));
        }
        MoovIoAchStringZeros { m }
    }
}

impl MoovIoAchFileControl {
    fn new() -> MoovIoAchFileControl {
        MoovIoAchFileControl {
            total_debit: 0,
            converters: MoovIoAchConverters,
        }
    }
}

impl fmt::Display for MoovIoAchFileControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "total_debit:{:?}",
            self.total_debit_entry_dollar_amount_in_file_field()
        )
    }
}

fn main() {
    let fc = MoovIoAchFileControl::new();
    println!("{}", fc);
}

