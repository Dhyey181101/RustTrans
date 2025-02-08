
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, Box<String>>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, Box<String>> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, Box::new(zero.repeat(i)));
    }
    out
}

fn moov_io_ach_get_string_zeros() -> &'static HashMap<usize, Box<String>> {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS
            .get_or_insert_with(|| moov_io_ach_populate_map(94, "0"))
    }
}

struct MoovIoAchAddenda18 {
    entry_detail_sequence_number: usize,
}

impl MoovIoAchAddenda18 {
    fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }

    fn numeric_field(&self, n: usize, max: u32) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            s.chars().rev().take(max as usize).collect()
        } else {
            let m = max as usize - l;
            let pad = moov_io_ach_get_string_zeros().get(&m).unwrap().to_string();
            pad + &s
        }
    }
}
