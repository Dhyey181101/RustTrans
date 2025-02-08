
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

unsafe fn get_moov_io_ach_string_zeros() -> &'static mut HashMap<usize, String> {
    MOOV_IO_ACH_STRING_ZEROS
        .get_or_insert_with(|| moov_io_ach_populate_map(94, "0"))
}

struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn format_simple_time(&self, s: &str) -> String {
        if s.is_empty() {
            self.string_field(s, 4)
        } else {
            s.to_owned()
        }
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s.chars().take(max).collect()
        } else {
            let m = max - ln;
            let mut pad = unsafe { get_moov_io_ach_string_zeros().get(&m).cloned() };
            if let Some(p) = pad.as_mut() {
                p.push_str(s);
                p.to_owned()
            } else {
                "0".repeat(m) + s
            }
        }
    }
}
