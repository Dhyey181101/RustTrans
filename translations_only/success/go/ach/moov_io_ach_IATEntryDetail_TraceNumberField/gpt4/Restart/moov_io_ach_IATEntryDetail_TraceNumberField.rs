
use std::collections::HashMap;
use std::str;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<i32, String>> = None;

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

struct MoovIoAchIATEntryDetail {
    trace_number: String,
    // Other fields omitted for brevity
}

impl MoovIoAchIATEntryDetail {
    fn trace_number_field(&self) -> String {
        self.string_field(&self.trace_number, 15)
    }

    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s.chars().take(max as usize).collect();
        }

        // Pad with preallocated string
        let m = (max - ln) as i32;
        unsafe {
            if let Some(ref map) = MOOV_IO_ACH_STRING_ZEROS {
                if let Some(pad) = map.get(&m) {
                    return pad.clone() + s;
                }
            }
        }

        // slow path
        "0".repeat(m as usize) + s
    }
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
    }
}
