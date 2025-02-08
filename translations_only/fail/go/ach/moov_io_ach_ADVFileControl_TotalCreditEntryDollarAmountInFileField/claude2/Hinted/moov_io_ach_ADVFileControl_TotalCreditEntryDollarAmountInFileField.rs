
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| populate_map(94, "0"));

struct MoovIoAchAdvFileControl {
    total_credit: i32,
}

struct MoovIoAchConverters;

impl MoovIoAchAdvFileControl {
    fn total_credit_entry_dollar_amount_in_file_field(&self) -> String {
        MoovIoAchConverters::numeric_field(self.total_credit, 20)
    }
}

impl MoovIoAchConverters {
    fn numeric_field(n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            s[s.len() - max as usize..].to_string()
        } else {
            let m = max - s.len() as u32;
            let mut pad = String::new();
            for _ in 0..m {
                pad.push('0');
            }
            pad + &s
        }
    }
}

fn populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        let mut s = String::new();
        for _ in 0..i {
            s.push_str(zero);
        }
        out.insert(i, s);
    }
    out
}

