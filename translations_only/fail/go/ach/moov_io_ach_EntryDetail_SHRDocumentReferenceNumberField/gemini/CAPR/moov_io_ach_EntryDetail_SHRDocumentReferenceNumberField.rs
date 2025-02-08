
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct MoovIoAchEntryDetail {
    pub identification_number: String,
}

pub struct MoovIoAchConverters;

impl MoovIoAchEntryDetail {
    pub fn shr_document_reference_number_field(&self) -> String {
        MoovIoAchConverters {}.string_field(&self.identification_number[4..15], 11)
    }
}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = moov_io_ach_string_zeros.get(&(m as i32)).unwrap();
        return format!("{}{}", pad, s);
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<i32, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, String::from_utf8(vec![b'0'; i as usize]).unwrap());
        }
        out
    };
}
