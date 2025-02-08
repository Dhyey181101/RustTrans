

use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<Box<HashMap<usize, Box<String>>>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> Box<HashMap<usize, Box<String>>> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, Box::new(zero.repeat(i)));
    }
    Box::new(out)
}

fn moov_io_ach_get_string_zeros() -> Box<HashMap<usize, Box<String>>> {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS
            .get_or_insert_with(|| moov_io_ach_populate_map(94, "0"))
            .clone()
    }
}

struct MoovIoAchAdvFileControl {
    block_count: usize,
}

impl MoovIoAchAdvFileControl {
    fn block_count_field(&self) -> String {
        self.numeric_field(self.block_count, 6)
    }

    fn numeric_field(&self, n: usize, max: u32) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            s.chars().rev().take(max as usize).collect()
        } else {
            let m = max as usize - l;
            let pad = Box::new(moov_io_ach_get_string_zeros().get(&m).unwrap_or(&Box::new("".to_string())).to_string());
            pad.to_string() + &s
        }
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
            let m = max as usize - l;
            let pad = Box::new(moov_io_ach_get_string_zeros().get(&m).unwrap_or(&Box::new("".to_string())).to_string());
            pad.to_string() + &s
        }
    }
}

