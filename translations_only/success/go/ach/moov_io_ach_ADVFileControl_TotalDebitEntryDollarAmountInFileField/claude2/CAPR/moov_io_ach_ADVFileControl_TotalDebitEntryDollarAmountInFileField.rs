
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
    if s.len() as u32 > max {
        s.drain(..s.len() - max as usize);
        s
    } else {
        let m = max - s.len() as u32;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&(m as i32)) {
            pad.to_owned() + &s
        } else {
            "0".repeat(m as usize) + &s
        }
    }
}

fn populate_map(max: i32, zero: String) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, "0".repeat(i as usize));
    }
    out
}

