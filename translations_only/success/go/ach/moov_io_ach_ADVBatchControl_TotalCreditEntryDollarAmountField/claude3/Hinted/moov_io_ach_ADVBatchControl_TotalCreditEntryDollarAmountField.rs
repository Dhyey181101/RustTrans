
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<Box<HashMap<usize, String>>> = Lazy::new(|| {
    Box::new(populate_map(94, "0".to_string()))
});

struct MoovIoAchAdvBatchControl {
    total_credit_entry_dollar_amount: i32,
}

impl MoovIoAchAdvBatchControl {
    fn total_credit_entry_dollar_amount_field(&self) -> String {
        numeric_field(self.total_credit_entry_dollar_amount, 20)
    }
}

fn numeric_field(n: i32, max: u32) -> String {
    let mut s = n.to_string();
    if s.len() > max as usize {
        s = s[s.len() - max as usize..].to_string();
    } else {
        let m = (max as usize) - s.len();
        let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap_or(&"".to_string()).clone();
        s = pad + &s;
    }
    s
}

fn populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}
