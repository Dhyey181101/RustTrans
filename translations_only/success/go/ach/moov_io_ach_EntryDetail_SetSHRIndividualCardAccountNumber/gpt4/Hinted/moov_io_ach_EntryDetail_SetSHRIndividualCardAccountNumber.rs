
use std::collections::HashMap;

static MOOV_IO_ACH_CHECKING_CREDIT: u32 = 22;

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchEntryDetail {
    individual_name: String,
}

impl MoovIoAchEntryDetail {
    fn set_shr_individual_card_account_number(&mut self, s: String) {
        self.individual_name = string_field(s, 22);
    }
}

fn string_field(s: String, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        return s.chars().take(max as usize).collect();
    }

    let m = (max - ln) as usize;
    if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
        return format!("{}{}", pad, s);
    }

    "0".repeat(m) + &s
}

#[macro_use]
extern crate lazy_static;

fn main() {}
