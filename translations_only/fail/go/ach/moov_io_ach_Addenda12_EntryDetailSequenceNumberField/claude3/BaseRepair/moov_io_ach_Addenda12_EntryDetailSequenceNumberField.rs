

use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchAddenda12 {
    entry_detail_sequence_number: usize,
}

impl MoovIoAchAddenda12 {
    fn entry_detail_sequence_number_field(&self) -> Box<String> {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }

    fn numeric_field(&self, n: usize, max: u32) -> Box<String> {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            let mut result = String::new();
            for c in s.chars().rev().take(max as usize) {
                result.insert(0, c);
            }
            Box::new(result)
        } else {
            let m = max as usize - l;
            unsafe {
                if let Some(ref map) = MOOV_IO_ACH_STRING_ZEROS {
                    if let Some(pad) = map.get(&m) {
                        let mut result = pad.to_owned();
                        result.push_str(&s);
                        Box::new(result)
                    } else {
                        Box::new("0".repeat(m) + &s)
                    }
                } else {
                    let map = moov_io_ach_populate_map(94, "0");
                    MOOV_IO_ACH_STRING_ZEROS = Some(map);
                    if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap().get(&m) {
                        let mut result = pad.to_owned();
                        result.push_str(&s);
                        Box::new(result)
                    } else {
                        Box::new("0".repeat(m) + &s)
                    }
                }
            }
        }
    }
}

