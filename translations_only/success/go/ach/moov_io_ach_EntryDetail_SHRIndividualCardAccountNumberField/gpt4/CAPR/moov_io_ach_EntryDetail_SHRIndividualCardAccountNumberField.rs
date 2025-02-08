
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = moov_io_ach_populate_map(94, "0");
}

const MOOV_IO_ACH_CHECKING_CREDIT: usize = 22;

struct MoovIoAchEntryDetail {
    individual_name: String,
}

impl MoovIoAchEntryDetail {
    fn shr_individual_card_account_number_field(&self) -> String {
        string_field(&self.individual_name, MOOV_IO_ACH_CHECKING_CREDIT)
    }
}

fn string_field(s: &str, max: usize) -> String {
    let ln = s.chars().count();
    if ln > max {
        return s.chars().take(max).collect::<String>();
    }

    let m = max - ln;
    if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
        return format!("{}{}", pad, s);
    }

    // slow path
    format!("{}{}", "0".repeat(m), s)
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

fn main() {}
