
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
        String::from(&s[s.len() - max..])
    } else {
        let m = max - s.len();
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
                moov_io_ach_numeric_field(n, max)
            }
        }
    }
}

struct MoovIoAchAddenda18 {
    sequence_number: usize,
    entry_detail_sequence_number: usize,
}

impl MoovIoAchAddenda18 {
    fn sequence_number_field(&self) -> String {
        moov_io_ach_numeric_field(self.sequence_number, 4)
    }
}
