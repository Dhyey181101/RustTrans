
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchFileControl {
    entry_hash: u64,
}

impl MoovIoAchFileControl {
    fn entry_hash_field(&self) -> String {
        self.numeric_field(self.entry_hash, 10)
    }
}

impl MoovIoAchFileControl {
    fn numeric_field(&self, n: u64, max: u8) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            return s[l - max as usize..].to_string();
        } else {
            let m = (max as usize) - l;
            unsafe {
                if MOOV_IO_ACH_STRING_ZEROS.is_none() {
                    MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(max as usize, "0"));
                }
                if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap().get(&m) {
                    return pad.to_owned() + &s;
                }
            }
            "0".repeat(m) + &s
        }
    }
}
