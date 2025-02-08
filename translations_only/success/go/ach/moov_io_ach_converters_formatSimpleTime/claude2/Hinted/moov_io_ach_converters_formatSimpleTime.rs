
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| populate_map(94));

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn format_simple_time(&self, s: &str) -> String {
        if s.is_empty() {
            self.string_field(s, 4)
        } else {
            s.to_string()
        }
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap().clone() + s
        }
    }
}

fn populate_map(max: usize) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out
}

