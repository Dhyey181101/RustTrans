
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

struct MoovIoAchFileControl {
    total_credit_entry_dollar_amount_in_file: i32,
}

impl MoovIoAchFileControl {
    fn total_credit_entry_dollar_amount_in_file_field(&self) -> String {
        self.numeric_field(self.total_credit_entry_dollar_amount_in_file, 12)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let mut s = n.to_string();
        let l = s.len();
        if l > max as usize {
            s = s.split_off(l - max as usize);
        } else {
            let m = (max as usize) - l;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                s = pad.clone() + &s;
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
