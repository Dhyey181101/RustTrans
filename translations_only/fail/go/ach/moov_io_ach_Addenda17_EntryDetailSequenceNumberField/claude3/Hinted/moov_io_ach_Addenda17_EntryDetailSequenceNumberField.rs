

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

fn numeric_field(n: u64, max: u32) -> String {
    let s = n.to_string();
    let l = s.len();
    if l > max as usize {
        String::from(&s[(l - max as usize)..])
    } else {
        let m = (max as usize) - l;
        let pad = moov_io_ach_get_string_zeros().get(&m).map_or("".to_string(), |s| s.to_string());
        format!("{}{}", pad, s)
    }
}

struct MoovIoAchAddenda17 {
    entry_detail_sequence_number: u64,
}

impl MoovIoAchAddenda17 {
    fn entry_detail_sequence_number_field(&self) -> String {
        numeric_field(self.entry_detail_sequence_number, 7)
    }
}

