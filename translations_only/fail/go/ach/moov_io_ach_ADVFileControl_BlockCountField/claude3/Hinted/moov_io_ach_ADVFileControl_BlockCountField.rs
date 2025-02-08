
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchAdvFileControl {
    block_count: u32,
}

impl MoovIoAchAdvFileControl {
    fn block_count_field(&self) -> String {
        self.numeric_field(self.block_count as usize, 6)
    }

    fn numeric_field(&self, n: usize, max: usize) -> String {
        let s = n.to_string();
        if s.len() > max {
            return String::from(&s[s.len() - max..]);
        } else {
            let m = max - s.len();
            unsafe {
                if MOOV_IO_ACH_STRING_ZEROS.is_none() {
                    MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
                }
                if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap().get(&m) {
                    return pad.to_owned() + &s;
                }
            }
            "0".repeat(m) + &s
        }
    }
}
