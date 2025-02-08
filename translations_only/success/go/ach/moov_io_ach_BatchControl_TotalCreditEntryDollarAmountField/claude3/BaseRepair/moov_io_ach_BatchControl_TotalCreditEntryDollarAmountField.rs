
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

struct MoovIoAchBatchControl {
    total_credit_entry_dollar_amount: i32,
}

impl MoovIoAchBatchControl {
    fn total_credit_entry_dollar_amount_field(&self) -> String {
        self.numeric_field(self.total_credit_entry_dollar_amount, 12)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let mut s = n.to_string();
        let l = s.len();
        if l > max as usize {
            s = s[l - max as usize..].to_string();
        } else {
            let m = (max as usize) - l;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                s = pad.to_string() + &s;
            } else {
                s = "0".repeat(m) + &s;
            }
        }
        s
    }
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchConverters {}
