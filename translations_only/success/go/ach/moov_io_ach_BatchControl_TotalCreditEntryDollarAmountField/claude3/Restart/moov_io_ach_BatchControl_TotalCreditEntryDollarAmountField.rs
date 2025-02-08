

use std::collections::HashMap;
use std::str::FromStr;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, Box<String>>> = Lazy::new(|| moov_io_ach_populate_map(94, Box::new("0".to_string())));

struct MoovIoAchBatchControl {
    total_credit_entry_dollar_amount: i32,
}

impl MoovIoAchBatchControl {
    fn total_credit_entry_dollar_amount_field(&self) -> String {
        self.numeric_field(self.total_credit_entry_dollar_amount, 12)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            s.chars().rev().take(max as usize).collect()
        } else {
            let m = (max as usize) - l;
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).map_or(Box::new("".to_string()), |x| x.clone());
            pad.to_string() + &s
        }
    }
}

fn moov_io_ach_populate_map(max: usize, zero: Box<String>) -> HashMap<usize, Box<String>> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, Box::new(zero.repeat(i)));
    }
    out
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn new() -> Box<MoovIoAchConverters> {
        Box::new(MoovIoAchConverters)
    }
}

