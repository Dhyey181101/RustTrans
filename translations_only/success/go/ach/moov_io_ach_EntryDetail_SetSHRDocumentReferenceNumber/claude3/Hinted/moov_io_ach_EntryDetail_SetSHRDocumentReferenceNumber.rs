
use std::collections::HashMap;
use std::str;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, Box<String>>> = Lazy::new(|| {
    moov_io_ach_populate_map(94, "0")
});

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, Box<String>> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, Box::new(zero.repeat(i)));
    }
    out
}

fn moov_io_ach_string_field(s: &str, max: usize) -> Box<String> {
    let ln = s.chars().count();
    if ln > max {
        return Box::new(s.chars().take(max).collect());
    }

    let m = max - ln;
    let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap().clone();
    let mut padded = pad;
    padded.push_str(s);
    padded
}

struct MoovIoAchEntryDetail {
    identification_number: Box<String>,
}

impl MoovIoAchEntryDetail {
    fn set_shr_document_reference_number(&mut self, s: &str) {
        let mut new_id = self.identification_number.to_string();
        new_id.push_str(&moov_io_ach_string_field(s, 11).to_string());
        self.identification_number = Box::new(new_id);
    }
}
