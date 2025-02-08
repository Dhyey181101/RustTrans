

use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, Box<str>>> = Lazy::new(|| moov_io_ach_populate_map(94, Box::from("0")));

fn moov_io_ach_populate_map(max: usize, zero: Box<str>) -> HashMap<usize, Box<str>> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, Box::from(zero.repeat(i)));
    }
    out
}

struct MoovIoAchEntryDetail {
    amount: i32,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchEntryDetail {
    fn amount_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.amount as i32, 10)
    }
}

struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            s.chars().rev().take(max as usize).collect()
        } else {
            let m = (max as usize) - l;
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap_or(&Box::from("")).clone();
            pad.into_string() + &s
        }
    }
}

