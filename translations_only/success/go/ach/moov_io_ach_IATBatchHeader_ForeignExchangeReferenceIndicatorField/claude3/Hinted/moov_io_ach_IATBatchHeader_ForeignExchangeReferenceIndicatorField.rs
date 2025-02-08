

use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

struct MoovIoAchIatBatchHeader {
    foreign_exchange_reference_indicator: i32,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchIatBatchHeader {
    fn foreign_exchange_reference_indicator_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.foreign_exchange_reference_indicator, 1)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            s.chars().rev().take(max as usize).collect()
        } else {
            let m = (max as usize) - l;
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap_or(&String::new()).to_string();
            pad + &s
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

