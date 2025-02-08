
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

fn moov_io_ach_numeric_field(n: u64, max: u32) -> String {
    let s = n.to_string();
    if s.len() > max as usize {
        return s[s.len() - max as usize..].to_string();
    } else {
        let m = (max as usize) - s.len();
        unsafe {
            if let Some(map) = &mut MOOV_IO_ACH_STRING_ZEROS {
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

struct MoovIoAchAddenda16 {
    entry_detail_sequence_number: u64,
}

impl MoovIoAchAddenda16 {
    fn entry_detail_sequence_number_field(&self) -> String {
        moov_io_ach_numeric_field(self.entry_detail_sequence_number, 7)
    }
}
