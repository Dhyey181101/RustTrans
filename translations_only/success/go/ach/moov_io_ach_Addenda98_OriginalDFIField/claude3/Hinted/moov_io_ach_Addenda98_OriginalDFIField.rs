
use std::collections::HashMap;
use std::str;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchAddenda98 {
    original_dfi: String,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda98 {
    fn original_dfi_field(&self) -> String {
        self.moov_io_ach_converters.string_field(&self.original_dfi, 8)
    }
}

struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s.chars().take(max as usize).collect();
        }

        let m = (max - ln) as usize;
        unsafe {
            if MOOV_IO_ACH_STRING_ZEROS.is_none() {
                MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
            }
            if let Some(ref map) = MOOV_IO_ACH_STRING_ZEROS {
                if let Some(pad) = map.get(&m) {
                    return pad.to_owned() + s;
                }
            }
        }

        "0".repeat(m) + s
    }
}
