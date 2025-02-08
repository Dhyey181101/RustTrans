
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<Box<HashMap<usize, Box<String>>>> = Lazy::new(|| {
    Box::new(moov_io_ach_populate_map(94, "0"))
});

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, Box<String>> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, Box::new(zero.repeat(i)));
    }
    out
}

struct MoovIoAchAddenda12 {
    entry_detail_sequence_number: usize,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda12 {
    fn entry_detail_sequence_number_field(&self) -> String {
        self.moov_io_ach_converters
            .numeric_field(self.entry_detail_sequence_number, 7)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: usize, max: u32) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            s.chars().rev().take(max as usize).collect()
        } else {
            let m = max as usize - l;
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap_or(&Box::new(String::new())).to_string();
            pad + &s
        }
    }
}
