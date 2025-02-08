
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, Box<String>>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, Box<String>> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, Box::from(zero.repeat(i)));
    }
    out
}

struct MoovIoAchBatchControl {
    entry_hash: usize,
}

impl MoovIoAchBatchControl {
    fn entry_hash_field(&self) -> Box<String> {
        self.numeric_field(self.entry_hash, 10)
    }

    fn numeric_field(&self, n: usize, max: u32) -> Box<String> {
        let s = n.to_string();
        if s.len() > max as usize {
            Box::from(s.chars().rev().take(max as usize).collect::<String>())
        } else {
            let m = max as usize - s.len();
            let pad = unsafe {
                MOOV_IO_ACH_STRING_ZEROS
                    .get_or_insert_with(|| moov_io_ach_populate_map(95, "0"))
                    .get(&m)
                    .cloned()
                    .unwrap_or_default()
            };
            Box::from(pad.to_string() + &s)
        }
    }
}
