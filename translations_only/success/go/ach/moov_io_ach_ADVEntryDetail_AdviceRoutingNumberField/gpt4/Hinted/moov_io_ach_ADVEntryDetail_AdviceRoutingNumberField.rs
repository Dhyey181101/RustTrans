
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

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

struct MoovIoAchADVEntryDetail {
    advice_routing_number: String,
}

impl MoovIoAchADVEntryDetail {
    fn advice_routing_number_field(&self) -> String {
        let ln = self.advice_routing_number.chars().count();
        if ln > 9 {
            return self.advice_routing_number.chars().take(9).collect();
        }

        // Pad with preallocated string
        let m = 9 - ln;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            return format!("{}{}", pad, self.advice_routing_number);
        }
        // slow path
        "0".repeat(m) + &self.advice_routing_number
    }
}
