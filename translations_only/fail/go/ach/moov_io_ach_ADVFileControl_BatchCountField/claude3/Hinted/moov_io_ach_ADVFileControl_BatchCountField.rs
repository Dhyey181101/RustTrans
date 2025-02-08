
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
    batch_count: u64,
}

impl MoovIoAchAdvFileControl {
    fn batch_count_field(&self) -> String {
        self.numeric_field(self.batch_count, 6)
    }

    fn numeric_field(&self, n: u64, max: u8) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            return String::from(&s[(l - max as usize)..]);
        } else {
            let m = (max as usize) - l;
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
