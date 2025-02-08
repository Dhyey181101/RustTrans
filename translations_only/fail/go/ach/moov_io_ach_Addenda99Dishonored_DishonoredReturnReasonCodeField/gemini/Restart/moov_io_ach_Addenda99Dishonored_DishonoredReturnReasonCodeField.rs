
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchAddenda99Dishonored {
    pub dishonored_return_reason_code: String,
}

impl MoovIoAchAddenda99Dishonored {
    pub fn dishonored_return_reason_code_field(&self) -> &str {
        &self.dishonored_return_reason_code
    }
}

#[derive(Debug)]
pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = MOOv_IO_ACH_STRING_ZEROS.get(&(m as i32)).unwrap();
        format!("{}{}", pad, s)
    }
}

lazy_static! {
    static ref MOOv_IO_ACH_STRING_ZEROS: HashMap<i32, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, String::from_str("0").unwrap().repeat(i as usize));
        }
        out
    };
}

impl Deref for MoovIoAchConverters {
    type Target = MoovIoAchConverters;

    fn deref(&self) -> &Self::Target {
        self
    }
}
