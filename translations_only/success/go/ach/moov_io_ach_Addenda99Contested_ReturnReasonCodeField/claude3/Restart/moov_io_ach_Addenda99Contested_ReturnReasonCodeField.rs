
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

fn moov_io_ach_string_field(s: &str, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        return s.chars().take(max as usize).collect();
    }

    let m = (max - ln) as usize;
    unsafe {
        if let Some(ref mut map) = MOOV_IO_ACH_STRING_ZEROS {
            if let Some(pad) = map.get(&m) {
                return pad.clone() + s;
            }
        } else {
            MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
        }
    }

    "0".repeat(m) + s
}

struct MoovIoAchAddenda99Contested {
    return_reason_code: String,
}

impl MoovIoAchAddenda99Contested {
    fn return_reason_code_field(&self) -> String {
        moov_io_ach_string_field(&self.return_reason_code, 2)
    }
}

struct MoovIoAchConverters {}
