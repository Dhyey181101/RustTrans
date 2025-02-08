
use std::collections::HashMap;

use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| HashMap::new());

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn format_simple_date(&self, s: &str) -> String {
        if s.is_empty() {
            self.string_field(s, 6)
        } else {
            s.to_string()
        }
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let mut out = String::with_capacity(max);
        let ln = s.len();
        if ln > max {
            out.push_str(&s[..max]);
        } else {
            out.push_str(s);
            for _ in ln..max {
                out.push('0');
            }
        }
        out
    }
}
