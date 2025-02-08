
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchAddenda18 {
    sequence_number: u64,
    entry_detail_sequence_number: u64,
}

impl MoovIoAchAddenda18 {
    fn sequence_number_field(&self) -> String {
        self.numeric_field(self.sequence_number, 4)
    }

    fn numeric_field(&self, n: u64, max: u8) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            return s[l - max as usize..].to_string();
        } else {
            let m = max as usize - l;
            unsafe {
                if let Some(ref map) = MOOV_IO_ACH_STRING_ZEROS {
                    if let Some(pad) = map.get(&m) {
                        return pad.to_owned() + &s;
                    }
                } else {
                    MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
                }
            }
            "0".repeat(m) + &s
        }
    }
}
