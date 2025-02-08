
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

struct MoovIoAchAdvFileControl {
    total_debit: i32,
}

impl MoovIoAchAdvFileControl {
    fn total_debit_entry_dollar_amount_in_file_field(&self) -> String {
        numeric_field(self.total_debit, 20)
    }
}

struct MoovIoAchConverters;

fn numeric_field(n: i32, max: u32) -> String {
    let mut s = n.to_string();
    if s.len() > max as usize {
        s.drain(..s.len() - max as usize);
        s
    } else {
        let mut pad = String::new();
        for _ in 0..(max - s.len() as u32) {
            pad.push('0');
        }
        pad + &s
    }
}

fn populate_map(max: i32, zero: String) -> HashMap<i32, String> {
    let mut map = HashMap::new();
    for i in 0..max {
        map.insert(i, zero.repeat(i as usize));
    }
    map
}

