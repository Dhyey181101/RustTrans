
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchAddenda17 {
    sequence_number: usize,
    entry_detail_sequence_number: usize,
}

impl MoovIoAchAddenda17 {
    fn sequence_number_field(&self) -> String {
        self.numeric_field(self.sequence_number, 4)
    }
}

impl MoovIoAchAddenda17 {
    fn numeric_field(&self, n: usize, max: u32) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            s.chars().rev().take(max as usize).collect()
        } else {
            let m = max as usize - l;
            unsafe {
                if let Some(ref map) = MOOV_IO_ACH_STRING_ZEROS {
                    if let Some(pad) = map.get(&m) {
                        pad.to_owned() + &s
                    } else {
                        "0".repeat(m) + &s
                    }
                } else {
                    let map = moov_io_ach_populate_map(94, "0");
                    MOOV_IO_ACH_STRING_ZEROS = Some(map);
                    if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap().get(&m) {
                        pad.to_owned() + &s
                    } else {
                        "0".repeat(m) + &s
                    }
                }
            }
        }
    }
}
