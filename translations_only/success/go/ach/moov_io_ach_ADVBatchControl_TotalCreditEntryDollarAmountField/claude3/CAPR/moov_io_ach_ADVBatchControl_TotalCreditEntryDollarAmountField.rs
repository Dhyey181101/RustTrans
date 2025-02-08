
use std::collections::HashMap;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

struct MoovIoAchAdvBatchControl {
    total_credit_entry_dollar_amount: i32,
}

impl MoovIoAchAdvBatchControl {
    fn total_credit_entry_dollar_amount_field(&self) -> String {
        moov_io_ach_numeric_field(self.total_credit_entry_dollar_amount, 20)
    }
}

fn moov_io_ach_numeric_field(n: i32, max: u32) -> String {
    let mut s = n.to_string();
    if s.len() > max as usize {
        s.drain(..s.len() - max as usize);
    } else {
        let m = (max as usize) - s.len();
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            s = pad.clone() + &s;
        } else {
            s = "0".repeat(m) + &s;
        }
    }
    s
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchConverters {}

use once_cell::sync::Lazy;
