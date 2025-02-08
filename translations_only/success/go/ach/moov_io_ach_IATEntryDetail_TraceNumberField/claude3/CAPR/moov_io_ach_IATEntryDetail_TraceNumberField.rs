

use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    let mut out = HashMap::with_capacity(94);
    for i in 0..94 {
        out.insert(i, "0".repeat(i));
    }
    out
});

struct MoovIoAchIatEntryDetail {
    trace_number: String,
}

impl MoovIoAchIatEntryDetail {
    fn trace_number_field(&self) -> String {
        let ln = self.trace_number.chars().count() as u32;
        if ln > 15 {
            return self.trace_number[..15].to_owned();
        }

        let m = (15 - ln) as usize;
        let zeros = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap();
        zeros.to_owned() + &self.trace_number
    }
}

