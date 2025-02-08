
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

struct MoovIoAchAddenda99Contested {
    dishonored_return_settlement_date: String,
}

impl MoovIoAchAddenda99Contested {
    fn dishonored_return_settlement_date_field(&self) -> String {
        let max = 3;
        let ln = self.dishonored_return_settlement_date.chars().count() as u32;
        if ln > max {
            return self.dishonored_return_settlement_date.chars().take(max as usize).collect();
        }

        let m = (max - ln) as usize;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            return pad.clone() + &self.dishonored_return_settlement_date;
        }

        "0".repeat(m) + &self.dishonored_return_settlement_date
    }
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out
}
