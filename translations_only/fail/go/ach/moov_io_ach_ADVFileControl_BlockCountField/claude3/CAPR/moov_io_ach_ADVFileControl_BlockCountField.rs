

use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, Box<str>>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, Box<str>> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i).into_boxed_str());
    }
    out
}

fn moov_io_ach_get_string_zeros() -> &'static HashMap<usize, Box<str>> {
    unsafe {
        if MOOV_IO_ACH_STRING_ZEROS.is_none() {
            MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
        }
        MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap()
    }
}

struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: usize, max: u32) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            s.chars().rev().take(max as usize).collect()
        } else {
            let m = (max as usize) - l;
            let pad = moov_io_ach_get_string_zeros().get(&m).map_or("".to_string(), |s| s.to_string());
            pad + &s
        }
    }
}

struct MoovIoAchAdvFileControl {
    block_count: usize,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAdvFileControl {
    fn block_count_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.block_count, 6)
    }
}

