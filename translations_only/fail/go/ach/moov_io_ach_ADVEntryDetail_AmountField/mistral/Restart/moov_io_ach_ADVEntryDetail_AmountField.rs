

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &[u8] = b"0000000000";

struct MoovIoAchAdvEntryDetail {
    amount: i32,
}

impl MoovIoAchAdvEntryDetail {
    fn amount_field(&self) -> String {
        MoovIoAchConverters::numeric_field(self, self.amount, 12)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(entry: &MoovIoAchAdvEntryDetail, n: i32, max: u32) -> String {
        let s = n.to_string();
        let len = s.len() as u32;
        if len > max {
            String::from(&s[(len - max) as usize..])
        } else {
            let padding_len = max - len;
            let padding = get_padding(padding_len as usize);
            padding.chars().chain(s.chars()).collect()
        }
    }
}

fn get_padding(len: usize) -> String {
    if len > ZEROS.len() {
        String::from_utf8(ZEROS[..len].to_vec()).unwrap()
    } else {
        String::from_utf8(ZEROS[..len].to_vec()).unwrap()
    }
}

impl fmt::Display for MoovIoAchAdvEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.amount_field())
    }
}

