

use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

struct MoovIoAchFileControl {
    total_debit: i32
}

impl MoovIoAchFileControl {
    fn total_debit_entry_dollar_amount_in_file_field(&self) -> String {
        numeric_field(self.total_debit, 12)
    }
}

struct MoovIoAchConverters;

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    if s.len() as u32 > max {
        s[s.len()-max as usize..].to_string()
    } else {
        let m = max - s.len() as u32;
        MOOV_IO_ACH_STRING_ZEROS.get(&(m as usize)).unwrap().clone() + &s
    }
}

fn populate_map(max: i32, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max as usize {
        out.insert(i, zero.repeat(i));
    }
    out
}

