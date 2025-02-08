
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<Box<HashMap<usize, Box<String>>>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> Box<HashMap<usize, Box<String>>> {
    let mut out = Box::new(HashMap::with_capacity(max));
    for i in 0..max {
        out.insert(i, Box::new(zero.repeat(i)));
    }
    out
}

fn moov_io_ach_string_zeros() -> &'static Box<HashMap<usize, Box<String>>> {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS
            .get_or_insert_with(|| moov_io_ach_populate_map(94, "0"))
    }
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
            let pad = moov_io_ach_string_zeros().get(&m).map(|x| x.to_string()).unwrap_or_default();
            pad + s
        }
    }
}
