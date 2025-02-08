
use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &[u8] = b"000000000000000000";

struct Addenda10 {
    foreign_payment_amount: i32,
    moov_io_ach_converters: Box<Converters>,
}

struct Converters {
    _priv: (),
}

impl Addenda10 {
    fn foreign_payment_amount_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.foreign_payment_amount, 18)
    }
}

impl Converters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l < max {
            let padding = (max - l) as usize;
            let zeros = ZEROS[..padding].to_vec();
            format!("{}{}", String::from_utf8(zeros).unwrap(), s)
        } else {
            s
        }
    }
}
