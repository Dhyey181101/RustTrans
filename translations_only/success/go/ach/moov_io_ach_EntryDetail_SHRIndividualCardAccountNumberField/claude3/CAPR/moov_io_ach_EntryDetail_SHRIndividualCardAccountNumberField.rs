
use std::collections::HashMap;
use std::str;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    moov_io_ach_populate_map(94, "0".to_string())
});

const MOOV_IO_ACH_CHECKING_CREDIT: u8 = 22;

impl MoovIoAchEntryDetail {
    fn shr_individual_card_account_number_field(&self) -> String {
        self.moov_io_ach_converters.string_field(&self.individual_name, 22)
    }
}

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap_or(&"0".repeat(m)).to_string();
        pad + s
    }
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchEntryDetail {
    individual_name: String,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters {}
