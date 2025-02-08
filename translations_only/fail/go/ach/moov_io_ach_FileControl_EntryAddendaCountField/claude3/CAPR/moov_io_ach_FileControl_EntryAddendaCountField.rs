

use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<Box<HashMap<usize, Box<String>>>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> Box<HashMap<usize, Box<String>>> {
    let mut out = Box::new(HashMap::with_capacity(max));
    for i in 0..max {
        out.insert(i, Box::new(zero.repeat(i)));
    }
    out
}

fn moov_io_ach_get_string_zeros() -> &'static Box<HashMap<usize, Box<String>>> {
    unsafe {
        if MOOV_IO_ACH_STRING_ZEROS.is_none() {
            MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
        }
        MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap()
    }
}

struct MoovIoAchFileControl {
    entry_addenda_count: usize,
}

impl MoovIoAchFileControl {
    fn entry_addenda_count_field(&self) -> String {
        self.numeric_field(self.entry_addenda_count, 8)
    }
}

impl MoovIoAchFileControl {
    fn numeric_field(&self, n: usize, max: u32) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            s[(l - max as usize)..].to_string()
        } else {
            let m = (max as usize) - l;
            let pad = moov_io_ach_get_string_zeros().get(&m).unwrap().to_string();
            pad + &s
        }
    }
}

struct MoovIoAchConverters;

