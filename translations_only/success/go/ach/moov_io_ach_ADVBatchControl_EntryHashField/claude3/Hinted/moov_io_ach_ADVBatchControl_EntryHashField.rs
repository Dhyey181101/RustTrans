
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i64, max: u32) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            return String::from(&s[(l - max as usize)..]);
        } else {
            let m = (max as usize) - l;
            unsafe {
                if let Some(map) = &MOOV_IO_ACH_STRING_ZEROS {
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

struct MoovIoAchAdvBatchControl {
    entry_hash: i64,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAdvBatchControl {
    fn entry_hash_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.entry_hash, 10)
    }
}
