
use std::collections::HashMap;
use std::str;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    let mut out = HashMap::with_capacity(94);
    for i in 0..94 {
        out.insert(i, "0".to_string().repeat(i));
    }
    out
});

struct MoovIoAchIATBatchHeader {
    effective_entry_date: String,
}

impl MoovIoAchIATBatchHeader {
    fn effective_entry_date_field(&self) -> String {
        string_field(&self.effective_entry_date, 6) // YYMMDD
    }
}

fn string_field(s: &str, max: usize) -> String {
    let ln = s.chars().count();
    if ln > max {
        return s.chars().take(max).collect();
    }

    let m = max - ln;
    if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
        return pad.to_owned() + s;
    }

    "0".repeat(m) + s
}

struct MoovIoAchConverters {}
