
use std::collections::HashMap;
use std::ops::Deref;
use std::str;
use once_cell::sync::Lazy as OtherLazy;

static MOOV_IO_ACH_STRING_ZEROS: OtherLazy<HashMap<usize, String>> = OtherLazy::new(|| {
    let mut out = HashMap::with_capacity(94);
    for i in 0..94 {
        out.insert(i, "0".repeat(i));
    }
    out
});

const MOOV_IO_ACH_CHECKING_CREDIT: u8 = 22;

struct moov_io_ach_EntryDetail {
    individual_name: String,
    moov_io_ach_converters: Box<moov_io_ach_converters>,
}

impl moov_io_ach_EntryDetail {
    fn shr_individual_card_account_number_field(&self) -> String {
        self.moov_io_ach_converters.string_field(&self.individual_name, 22)
    }
}

struct moov_io_ach_converters {}

impl moov_io_ach_converters {
    fn string_field(&self, s: &str, max: u8) -> String {
        let ln = s.chars().count() as u8;
        if ln > max {
            return s.chars().take(max as usize).collect();
        }

        let m = (max - ln) as usize;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.deref().get(&m) {
            return pad.to_owned() + s;
        }

        "0".repeat(m) + s
    }
}
