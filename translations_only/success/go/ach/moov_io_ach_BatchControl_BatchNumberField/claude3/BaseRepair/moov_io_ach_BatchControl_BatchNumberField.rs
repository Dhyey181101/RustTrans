
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

struct MoovIoAchBatchControl {
    batch_number: i32,
}

impl MoovIoAchBatchControl {
    fn batch_number_field(&self) -> String {
        self.numeric_field(self.batch_number, 7)
    }
}

impl MoovIoAchBatchControl {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            s.chars().rev().take(max as usize).collect()
        } else {
            let m = (max as usize) - l;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                pad.to_string() + &s
            } else {
                "0".repeat(m) + &s
            }
        }
    }
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}
