

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

struct MoovIoAchADVBatchControl {
    entry_addenda_count: usize,
}

impl MoovIoAchADVBatchControl {
    fn entry_addenda_count_field(&self) -> String {
        moov_io_ach_numeric_field(self.entry_addenda_count, 6)
    }
}

fn moov_io_ach_numeric_field(n: usize, max: u32) -> String {
    let s = n.to_string();
    let l = s.len();
    if l > max as usize {
        s.chars().rev().take(max as usize).collect()
    } else {
        let m = (max as usize) - l;
        let pad = moov_io_ach_get_string_zeros().get(&m).map(|s| s.to_string()).unwrap_or_else(|| "".to_string());
        pad + &s
    }
}

