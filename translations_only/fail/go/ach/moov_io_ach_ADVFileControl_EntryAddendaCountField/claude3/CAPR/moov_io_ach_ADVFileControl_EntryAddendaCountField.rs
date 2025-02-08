
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

fn moov_io_ach_numeric_field(n: usize, max: usize) -> String {
    let s = n.to_string();
    if s.len() > max {
        return s[s.len() - max..].to_string();
    } else {
        let m = max - s.len();
        unsafe {
            if MOOV_IO_ACH_STRING_ZEROS.is_none() {
                MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
            }
            if let Some(map) = &MOOV_IO_ACH_STRING_ZEROS {
                if let Some(pad) = map.get(&m) {
                    return pad.to_owned() + &s;
                }
            }
        }
        // slow path
        "0".repeat(m) + &s
    }
}

struct MoovIoAchAdvFileControl {
    entry_addenda_count: usize,
}

impl MoovIoAchAdvFileControl {
    fn entry_addenda_count_field(&self) -> String {
        moov_io_ach_numeric_field(self.entry_addenda_count, 8)
    }
}
