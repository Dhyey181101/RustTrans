
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchBatchControl {
    entry_hash: usize,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchBatchControl {
    fn entry_hash_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.entry_hash, 10)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: usize, max: u32) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            String::from(&s[(l - max as usize)..])
        } else {
            let m = (max as usize) - l;
            unsafe {
                if let Some(ref map) = MOOV_IO_ACH_STRING_ZEROS {
                    if let Some(pad) = map.get(&m) {
                        pad.to_owned() + &s
                    } else {
                        "0".repeat(m) + &s
                    }
                } else {
                    let map = moov_io_ach_populate_map(95, "0");
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
