

use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<Box<HashMap<usize, String>>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> Box<HashMap<usize, String>> {
    let mut out = Box::new(HashMap::with_capacity(max));
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

unsafe fn moov_io_ach_string_zeros() -> &'static Box<HashMap<usize, String>> {
    MOOV_IO_ACH_STRING_ZEROS
        .get_or_insert_with(|| moov_io_ach_populate_map(94, "0"))
}

struct MoovIoAchAdvFileControl {
    batch_count: usize,
}

impl MoovIoAchAdvFileControl {
    fn batch_count_field(&self) -> String {
        moov_io_ach_numeric_field(self.batch_count, 6)
    }
}

fn moov_io_ach_numeric_field(n: usize, max: usize) -> String {
    let s = n.to_string();
    if s.len() > max {
        String::from(&s[s.len() - max..])
    } else {
        let m = max - s.len();
        let pad = unsafe { moov_io_ach_string_zeros().get(&m).map_or(String::new(), |v| v.to_string()) };
        pad + &s
    }
}

