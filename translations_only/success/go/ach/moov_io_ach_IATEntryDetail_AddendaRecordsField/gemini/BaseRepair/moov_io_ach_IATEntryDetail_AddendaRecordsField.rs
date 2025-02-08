
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct MoovIoAchIATEntryDetail {
    pub addenda_records: i32,
}

impl MoovIoAchIATEntryDetail {
    pub fn addenda_records_field(&self) -> String {
        self.numeric_field(self.addenda_records, 4)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return (&s[(s.len() - max as usize)..]).to_string();
        } else {
            let m = max - s.len() as u32;
            let pad = moov_io_ach_string_zeros.get(&m).unwrap();
            return format!("{}{}", pad, s);
        }
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<u32, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, String::from("0").repeat(i as usize));
        }
        out
    };
}
