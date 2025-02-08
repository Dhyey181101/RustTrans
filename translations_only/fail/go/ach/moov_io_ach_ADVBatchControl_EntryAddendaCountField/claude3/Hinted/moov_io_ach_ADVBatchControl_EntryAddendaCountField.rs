
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
    fn numeric_field(&self, n: u64, max: u32) -> String {
        let s = n.to_string();
        if s.len() > max as usize {
            return String::from(&s[(s.len() - max as usize)..]);
        } else {
            let m = (max as usize) - s.len();
            unsafe {
                if MOOV_IO_ACH_STRING_ZEROS.is_none() {
                    MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
                }
                if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap().get(&m) {
                    return pad.to_owned() + &s;
                }
            }
            // slow path
            return "0".repeat(m) + &s;
        }
    }
}

struct MoovIoAchAdvBatchControl {
    entry_addenda_count: u64,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAdvBatchControl {
    fn entry_addenda_count_field(&self) -> String {
        self.moov_io_ach_converters
            .numeric_field(self.entry_addenda_count, 6)
    }
}
