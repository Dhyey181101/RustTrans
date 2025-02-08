
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
    return_settlement_date: String,
}

impl MoovIoAchAddenda99Dishonored {
    fn return_settlement_date_field(&self) -> String {
        let ln = self.return_settlement_date.chars().count();
        if ln > 3 {
            return self.return_settlement_date.chars().take(3).collect();
        }

        let m = 3 - ln;
        unsafe {
            if MOOV_IO_ACH_STRING_ZEROS.is_none() {
                MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
            }
            if let Some(ref pad_map) = MOOV_IO_ACH_STRING_ZEROS {
                if let Some(pad) = pad_map.get(&m) {
                    return format!("{}{}", pad, self.return_settlement_date);
                }
            }
        }

        // slow path
        "0".repeat(m) + &self.return_settlement_date
    }
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
    }
}
