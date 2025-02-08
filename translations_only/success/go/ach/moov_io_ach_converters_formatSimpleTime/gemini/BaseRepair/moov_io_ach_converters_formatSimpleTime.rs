
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn format_simple_time(&self, s: &str) -> String {
        if s.is_empty() {
            return self.string_field(s, 4);
        }
        s.to_string()
    }

    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = MOOV_IO_ACH_STRING_ZEROS.get(&(m as i32)).unwrap();
        format!("{}{}", pad, s)
    }
}

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<i32, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, String::from_utf8(vec![b'0'; i as usize]).unwrap());
        }
        out
    };
}
