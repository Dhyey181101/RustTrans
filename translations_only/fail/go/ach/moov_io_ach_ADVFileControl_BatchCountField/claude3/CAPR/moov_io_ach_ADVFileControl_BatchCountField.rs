
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<Box<HashMap<usize, String>>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> Box<HashMap<usize, String>> {
    let mut out = Box::new(HashMap::with_capacity(max));
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

fn moov_io_ach_string_zeros() -> Box<HashMap<usize, String>> {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS
            .get_or_insert_with(|| moov_io_ach_populate_map(94, "0"))
            .clone()
    }
}

struct MoovIoAchAdvFileControl {
    batch_count: usize,
}

impl MoovIoAchAdvFileControl {
    fn batch_count_field(&self) -> String {
        moov_io_ach_numeric_field(self.batch_count, 6)
    }
}

fn moov_io_ach_numeric_field(n: usize, max: u32) -> String {
    let s = n.to_string();
    let l = s.len();
    if l > max as usize {
        s.chars().rev().take(max as usize).collect()
    } else {
        let m = (max as usize) - l;
        let zeros = moov_io_ach_string_zeros();
        zeros.get(&m).unwrap_or(&"".repeat(m)).to_owned() + &s
    }
}
