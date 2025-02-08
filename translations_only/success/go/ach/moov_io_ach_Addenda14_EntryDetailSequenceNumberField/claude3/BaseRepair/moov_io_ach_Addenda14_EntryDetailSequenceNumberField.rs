
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> =
    Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

fn moov_io_ach_entry_detail_sequence_number_field(addenda14: &Box<MoovIoAchAddenda14>) -> String {
    addenda14.numeric_field(addenda14.entry_detail_sequence_number, 7)
}

fn moov_io_ach_numeric_field(n: i64, max: u32) -> String {
    let s = n.to_string();
    let l = s.len();
    if l > max as usize {
        s.chars().skip(l - max as usize).collect()
    } else {
        let m = (max as usize) - l;
        let pad = MOOV_IO_ACH_STRING_ZEROS
            .get(&m)
            .unwrap_or(&"".to_string())
            .clone();
        pad + &s
    }
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchAddenda14 {
    entry_detail_sequence_number: i64,
}

impl MoovIoAchAddenda14 {
    fn numeric_field(&self, n: i64, max: u32) -> String {
        moov_io_ach_numeric_field(n, max)
    }
}
