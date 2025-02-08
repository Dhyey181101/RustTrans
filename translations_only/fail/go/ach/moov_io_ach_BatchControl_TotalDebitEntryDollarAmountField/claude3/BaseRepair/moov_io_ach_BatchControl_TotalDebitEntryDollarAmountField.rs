
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchBatchControl {
    total_debit_entry_dollar_amount: usize,
}

impl MoovIoAchBatchControl {
    fn total_debit_entry_dollar_amount_field(&self) -> String {
        self.numeric_field(self.total_debit_entry_dollar_amount, 12)
    }

    fn numeric_field(&self, n: usize, max: u32) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            s.chars().rev().take(max as usize).collect()
        } else {
            let m = (max as usize) - l;
            let pad = unsafe {
                if let Some(ref map) = MOOV_IO_ACH_STRING_ZEROS {
                    map.get(&m).cloned().unwrap_or_else(|| "0".repeat(m))
                } else {
                    let map = moov_io_ach_populate_map(95, "0");
                    MOOV_IO_ACH_STRING_ZEROS = Some(map);
                    MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap().get(&m).cloned().unwrap()
                }
            };
            pad + &s
        }
    }
}
