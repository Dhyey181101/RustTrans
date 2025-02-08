
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;

pub struct MoovIoAchAdvBatchControl {
    pub total_credit_entry_dollar_amount: i32,
}

impl MoovIoAchAdvBatchControl {
    pub fn total_credit_entry_dollar_amount_field(&self) -> String {
        self.numeric_field(self.total_credit_entry_dollar_amount, 20)
    }
}

impl MoovIoAchAdvBatchControl {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() - max as usize)..].to_string();
        } else {
            let m = max as usize - s.len();
            let pad = moov_io_ach_string_zeros.get(&m).unwrap();
            return format!("{}{}", pad, s);
        }
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<usize, String> =
        moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

#[derive(Debug)]
pub struct MoovIoAchConverters {}

impl fmt::Display for MoovIoAchConverters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MoovIoAchConverters")
    }
}

