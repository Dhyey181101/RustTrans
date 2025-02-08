
use std::collections::HashMap;

static MOOV_IO_ACH_CHECKING_CREDIT: u32 = 22;

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<u32, String> = moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: u32, zero: &str) -> HashMap<u32, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

struct MoovIoAchEntryDetail {
    individual_name: String,
}

impl MoovIoAchEntryDetail {
    fn shr_individual_card_account_number_field(&self) -> String {
        MoovIoAchConverters::string_field(&self.individual_name, MOOV_IO_ACH_CHECKING_CREDIT)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s.chars().take(max as usize).collect();
        }

        let m = max - ln;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            return format!("{}{}", pad, s);
        }

        // slow path
        "0".repeat(m as usize) + s
    }
}

#[macro_use]
extern crate lazy_static;

fn main() {
    // Example usage
    let entry_detail = MoovIoAchEntryDetail {
        individual_name: "John Doe".to_string(),
    };
    println!("{}", entry_detail.shr_individual_card_account_number_field());
}
