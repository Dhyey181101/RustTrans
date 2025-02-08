
use std::collections::HashMap;

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
            let mut result = "0".repeat(max - ln);
            result.push_str(s);
            result
        }
    }
}
