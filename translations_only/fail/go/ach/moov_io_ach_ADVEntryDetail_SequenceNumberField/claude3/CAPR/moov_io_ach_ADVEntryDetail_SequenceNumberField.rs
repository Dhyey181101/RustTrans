
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, Box<String>>> = Lazy::new(|| moov_io_ach_populate_map(94, Box::new("0".to_string())));

struct MoovIoAchAdvEntryDetail {
    sequence_number: usize,
}

impl MoovIoAchAdvEntryDetail {
    fn sequence_number_field(&self) -> String {
        numeric_field(self.sequence_number, 4)
    }
}

fn numeric_field(n: usize, max: u32) -> String {
    let s = n.to_string();
    let l = s.len();
    if l > max as usize {
        s.chars().rev().take(max as usize).collect()
    } else {
        let m = (max as usize) - l;
        let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap_or(&Box::new("".to_string())).to_string();
        pad + &s
    }
}

fn moov_io_ach_populate_map(max: usize, zero: Box<String>) -> HashMap<usize, Box<String>> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, Box::new(zero.repeat(i)));
    }
    out
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: usize, max: u32) -> String {
        numeric_field(n, max)
    }
}
