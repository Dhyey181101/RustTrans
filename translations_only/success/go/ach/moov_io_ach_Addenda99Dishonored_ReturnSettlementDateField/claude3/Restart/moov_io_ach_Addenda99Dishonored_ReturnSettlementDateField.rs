
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

fn moov_io_ach_string_zeros() -> &'static HashMap<usize, String> {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS
            .get_or_insert_with(|| moov_io_ach_populate_map(94, "0"))
    }
}

struct MoovIoAchAddenda99Dishonored {
    return_settlement_date: String,
}

impl MoovIoAchAddenda99Dishonored {
    fn return_settlement_date_field(&self) -> String {
        self.string_field(&self.return_settlement_date, 3)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s.chars().take(max).collect();
        }

        let m = max - ln;
        let pad = moov_io_ach_string_zeros().get(&m).cloned().unwrap_or_else(|| {
            let mut zeros = String::with_capacity(m);
            zeros.push_str(&"0".repeat(m));
            zeros
        });
        pad + s
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn new() -> Box<Self> {
        Box::new(MoovIoAchConverters {})
    }
}
