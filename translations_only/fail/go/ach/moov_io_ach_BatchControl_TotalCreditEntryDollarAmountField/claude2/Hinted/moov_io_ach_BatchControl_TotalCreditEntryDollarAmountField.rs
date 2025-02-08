
use std::collections::HashMap;
use std::str;

use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| {
    moov_io_ach_populate_map(94, "0".to_string())
});

struct MoovIoAchBatchControl {
    total_credit: i32,
    converters: Option<Box<MoovIoAchConverters>>,
}

struct MoovIoAchConverters;

impl MoovIoAchBatchControl {
    fn total_credit_entry_dollar_amount_field(&self) -> String {
        self.converters.as_ref().unwrap().numeric_field(self.total_credit, 12)
    }
}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            s[s.len() - max as usize..].to_string()
        } else {
            let m = max - s.len() as u32;
            let pad = MOOV_IO_ACH_STRINGZEROS.get(&(m as i32)).unwrap();
            format!("{}{}", pad, s)
        }
    }
}

fn moov_io_ach_populate_map(max: i32, zero: String) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, str::repeat(&zero, i as usize));
    }
    out
}

