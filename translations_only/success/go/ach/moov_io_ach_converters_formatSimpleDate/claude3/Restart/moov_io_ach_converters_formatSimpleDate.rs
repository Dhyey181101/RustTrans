
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

unsafe fn get_moov_io_ach_string_zeros() -> &'static HashMap<usize, String> {
    if MOOV_IO_ACH_STRING_ZEROS.is_none() {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(95, "0"));
    }
    MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap()
}

struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn format_simple_date(&self, s: &str) -> String {
        if s.is_empty() {
            self.string_field(s, 6)
        } else {
            s.to_string()
        }
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s.chars().take(max).collect()
        } else {
            let m = max - ln;
            let zeros = unsafe { get_moov_io_ach_string_zeros() };
            if let Some(pad) = zeros.get(&m) {
                pad.to_string() + s
            } else {
                "0".repeat(m) + s
            }
        }
    }
}
