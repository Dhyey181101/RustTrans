
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchAddenda10 {
    pub foreign_payment_amount: i32,
}

impl MoovIoAchAddenda10 {
    pub fn foreign_payment_amount_field(&self) -> String {
        let s = self.numeric_field(self.foreign_payment_amount, 18);
        s
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            s[(s.len() - max as usize)..].to_string()
        } else {
            let m = max - s.len() as u32;
            let pad = MOOv_IO_ACH_STRING_ZEROS.get(&m).unwrap().to_string();
            format!("{}{}", pad, s)
        }
    }
}

lazy_static! {
    static ref MOOv_IO_ACH_STRING_ZEROS: HashMap<u32, String> =
        moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<u32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i as u32, zero.repeat(i as usize));
    }
    out
}
