
use std::collections::HashMap;

use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| populate_map(94, "0"));

const MOOV_IO_ACH_CHECKINGCREDIT: usize = 22;

struct MoovIoAchEntryDetail {
    individual_name: String,
}

impl MoovIoAchEntryDetail {
    fn set_shr_individual_card_account_number(&mut self, s: String) {
        self.individual_name = string_field(&s, MOOV_IO_ACH_CHECKINGCREDIT);
    }
}

struct MoovIoAchConverters;

fn string_field(s: &String, max: usize) -> String {
    let ln = s.len();
    if ln > max {
        s[..max].to_string()
    } else {
        let m = max - ln;
        let pad = MOOV_IO_ACH_STRINGZEROS.get(&m).cloned().unwrap_or_else(|| "0".repeat(m));
        format!("{}{}", pad, s)
    }
}

fn populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

