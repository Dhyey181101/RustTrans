

use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<Box<HashMap<usize, String>>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> Box<HashMap<usize, String>> {
    let mut out = Box::new(HashMap::with_capacity(max));
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchFileControl {
    block_count: usize,
}

impl MoovIoAchFileControl {
    fn block_count_field(&self) -> String {
        self.numeric_field(self.block_count, 6)
    }

    fn numeric_field(&self, n: usize, max: u32) -> String {
        let s = n.to_string();
        if s.len() > max as usize {
            return String::from(&s[(s.len() - max as usize)..]);
        } else {
            let m = max as usize - s.len();
            let mut zeros = unsafe { MOOV_IO_ACH_STRING_ZEROS.get_or_insert_with(|| moov_io_ach_populate_map(94, "0")) };
            if let Some(pad) = zeros.get(&m) {
                return pad.to_owned() + &s;
            }
            // slow path
            return "0".repeat(m) + &s;
        }
    }
}

unsafe fn init() {
    MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
}

