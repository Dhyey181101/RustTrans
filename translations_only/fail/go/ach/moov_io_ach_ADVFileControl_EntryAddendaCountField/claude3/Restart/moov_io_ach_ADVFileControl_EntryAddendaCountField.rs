
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
    entry_addenda_count: usize,
}

impl MoovIoAchAdvFileControl {
    fn entry_addenda_count_field(&self) -> String {
        self.numeric_field(self.entry_addenda_count, 8)
    }
}

impl MoovIoAchAdvFileControl {
    fn numeric_field(&self, n: usize, max: u32) -> String {
        let mut s = n.to_string();
        if s.len() > max as usize {
            s = s.split_off(s.len() - max as usize);
        } else {
            let m = max as usize - s.len();
            unsafe {
                if MOOV_IO_ACH_STRING_ZEROS.is_none() {
                    MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
                }
                if let Some(map) = &MOOV_IO_ACH_STRING_ZEROS {
                    if let Some(pad) = map.get(&m) {
                        s = pad.to_owned() + &s;
                    } else {
                        s = "0".repeat(m) + &s;
                    }
                }
            }
        }
        s
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn new() -> Box<Self> {
        Box::new(MoovIoAchConverters)
    }
}
