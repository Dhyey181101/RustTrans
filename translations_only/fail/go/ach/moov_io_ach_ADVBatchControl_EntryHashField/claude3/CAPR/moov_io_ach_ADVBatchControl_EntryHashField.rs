
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, Box<str>> = {
        let mut map = HashMap::with_capacity(94);
        for i in 0..94 {
            map.insert(i, Box::from("0".repeat(i)));
        }
        map
    };
}

struct MoovIoAchADVBatchControl {
    entry_hash: usize,
}

impl MoovIoAchADVBatchControl {
    fn entry_hash_field(&self) -> String {
        self.numeric_field(self.entry_hash, 10)
    }

    fn numeric_field(&self, n: usize, max: u32) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            s.chars().rev().take(max as usize).collect()
        } else {
            let m = max as usize - l;
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).cloned().unwrap_or_else(|| Box::from(""));
            pad.into_string() + &s
        }
    }
}

struct MoovIoAchConverters {}
