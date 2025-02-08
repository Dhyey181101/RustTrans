
use std::collections::HashMap;
use std::str;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for i in 0..94 {
        map.insert(i, "0".repeat(i as usize));
    }
    map
});

struct MoovIoAchAdvBatchControl {
    total_debit: i32
}

impl MoovIoAchAdvBatchControl {
    fn total_debit_entry_dollar_amount_field(&self) -> String {
        numeric_field(self.total_debit, 20)
    }
}

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    if s.len() as u32 > max {
        s[s.len()-max as usize..].to_string()
    } else {
        let m = max - s.len() as u32;
        let pad = "0".repeat(m as usize);
        pad + &s
    }
}

struct MoovIoAchConverters;

