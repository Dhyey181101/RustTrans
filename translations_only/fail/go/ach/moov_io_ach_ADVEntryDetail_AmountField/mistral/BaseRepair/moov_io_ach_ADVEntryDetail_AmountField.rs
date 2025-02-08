

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
    fn numeric_field(detail: &MoovIoAchAdvEntryDetail, n: i32, max: u32) -> String {
        let s = n.to_string();
        let len = s.len() as u32;
        if len > max {
            s[(len - max) as usize..].to_string()
        } else {
            let padding_len = max - len;
            let padding = get_padding(padding_len as usize);
            format!("{}{}", padding, s)
        }
    }
}

fn get_padding(len: usize) -> String {
    if len > ZEROS.len() {
        ZEROS.iter()
            .take(ZEROS.len())
            .cloned()
            .chain(std::iter::repeat(0u8).take(len - ZEROS.len()))
            .map(|c| c as char)
            .collect()
    } else {
        str::from_utf8(&ZEROS[..len]).unwrap().to_string()
    }
}

struct MoovIoAchPopulateMap;

impl MoovIoAchPopulateMap {
    fn populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
        let mut map = HashMap::new();
        for i in 0..=max {
            let value = "0".repeat(i as usize);
            map.insert(i, value);
        }
        map
    }
}

impl fmt::Display for MoovIoAchAdvEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.amount_field())
    }
}

fn main() {
    let mut map = MoovIoAchPopulateMap::populate_map(94, "0");
    let mut adv_entry_detail = MoovIoAchAdvEntryDetail { amount: 123456 };
    println!("{}", adv_entry_detail);
}

