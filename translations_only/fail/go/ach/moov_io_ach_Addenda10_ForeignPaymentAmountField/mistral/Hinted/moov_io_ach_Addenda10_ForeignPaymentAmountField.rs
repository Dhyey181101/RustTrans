

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
            let pad = MoovIoAchAddenda10::get_zeros(m as usize);
            return pad + &s;
        }
    }
}

struct MoovIoAchAddenda10 {
    foreign_payment_amount: i32,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda10 {
    fn new() -> MoovIoAchAddenda10 {
        MoovIoAchAddenda10 {
            foreign_payment_amount: 0,
            moov_io_ach_converters: Box::new(MoovIoAchConverters),
        }
    }

    fn foreign_payment_amount_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.foreign_payment_amount, 18)
    }

    fn get_zeros(n: usize) -> String {
        let mut out = HashMap::new();
        for i in 0..n {
            out.insert(i, "0".repeat(i));
        }
        out[&(n - 1)].clone()
    }
}

impl fmt::Display for MoovIoAchAddenda10 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: {}, TypeCode: Addenda10 types code '10', Transaction Type Code: {:?}, Foreign Payment Amount: ${}",
            "client defined string",
            "transaction type code",
            self.foreign_payment_amount_field()
        )
    }
}

fn main() {
    let addenda10 = MoovIoAchAddenda10::new();
    println!("{}", addenda10);
}

