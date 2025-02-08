
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

struct MoovIoAchAddenda99Contested {
    original_receiving_dfi_identification: String,
}

impl MoovIoAchAddenda99Contested {
    fn original_receiving_dfi_identification_field(&self) -> String {
        let mut s = self.original_receiving_dfi_identification.clone();
        if s.len() > 8 {
            s.truncate(8);
        } else {
            s = MOOV_IO_ACH_STRING_ZEROS.get(&(8 - s.len())).unwrap().clone() + &s;
        }
        s
    }
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}
