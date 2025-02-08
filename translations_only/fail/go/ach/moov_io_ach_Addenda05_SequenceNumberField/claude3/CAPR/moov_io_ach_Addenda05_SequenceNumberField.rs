

use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<Box<HashMap<usize, String>>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> Box<HashMap<usize, String>> {
    let mut out = Box::new(HashMap::with_capacity(max));
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

fn moov_io_ach_get_string_zeros() -> &'static Box<HashMap<usize, String>> {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS
            .get_or_insert_with(|| moov_io_ach_populate_map(94, "0"))
    }
}

struct MoovIoAchAddenda05 {
    sequence_number: usize,
    entry_detail_sequence_number: usize,
}

impl MoovIoAchAddenda05 {
    fn sequence_number_field(&self) -> String {
        self.numeric_field(self.sequence_number, 4)
    }

    fn numeric_field(&self, n: usize, max: u32) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            s.chars().rev().take(max as usize).collect()
        } else {
            let m = max as usize - l;
            let pad = moov_io_ach_get_string_zeros().get(&m).map_or("".to_string(), |s| s.to_string());
            pad + &s
        }
    }
}

