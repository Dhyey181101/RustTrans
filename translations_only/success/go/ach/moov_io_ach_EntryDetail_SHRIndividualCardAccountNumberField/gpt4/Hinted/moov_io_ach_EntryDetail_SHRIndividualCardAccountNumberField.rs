
use std::collections::HashMap;

static MOOV_IO_ACH_CHECKING_CREDIT: usize = 22;

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
    fn shr_individual_card_account_number_field(&self) -> String {
        let ln = self.individual_name.chars().count();
        if ln > MOOV_IO_ACH_CHECKING_CREDIT {
            return self.individual_name.chars().take(MOOV_IO_ACH_CHECKING_CREDIT).collect::<String>();
        }

        let m = MOOV_IO_ACH_CHECKING_CREDIT - ln;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            return pad.clone() + &self.individual_name;
        }

        // slow path
        "0".repeat(m) + &self.individual_name
    }
}

#[macro_use]
extern crate lazy_static;
