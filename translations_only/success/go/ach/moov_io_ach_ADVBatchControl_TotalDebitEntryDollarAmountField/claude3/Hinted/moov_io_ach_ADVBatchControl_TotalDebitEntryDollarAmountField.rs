

use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, Box<str>>> = Lazy::new(|| moov_io_ach_populate_map(94, Box::from("0")));

struct MoovIoAchAdvBatchControl {
    total_debit_entry_dollar_amount: i32,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAdvBatchControl {
    fn total_debit_entry_dollar_amount_field(&self) -> Box<str> {
        self.moov_io_ach_converters
            .numeric_field(self.total_debit_entry_dollar_amount, 20)
    }
}

struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> Box<str> {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            Box::from(s.chars().rev().take(max as usize).collect::<String>())
        } else {
            let m = (max as usize) - l;
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap_or(&Box::from("".to_string())).to_string();
            Box::from(pad + &s)
        }
    }
}

fn moov_io_ach_populate_map(max: usize, zero: Box<str>) -> HashMap<usize, Box<str>> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, Box::from(zero.to_string().repeat(i)));
    }
    out
}

