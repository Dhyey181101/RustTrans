
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchAddenda99Dishonored {
    dishonored_return_reason_code: String,
    converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda99Dishonored {
    fn new(dishonored_return_reason_code: String) -> Self {
        Self {
            dishonored_return_reason_code,
            converters: Box::new(MoovIoAchConverters {}),
        }
    }

    fn dishonored_return_reason_code_field(&self) -> String {
        self.converters.string_field(&self.dishonored_return_reason_code, 3)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s.chars().take(max).collect()
        } else {
            let m = max - ln;
            unsafe {
                if let Some(map) = &MOOV_IO_ACH_STRING_ZEROS {
                    if let Some(pad) = map.get(&m) {
                        return pad.clone() + s;
                    }
                }
            }
            "0".repeat(m) + s
        }
    }
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
    }
}
