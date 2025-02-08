

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &[u8] = b"0000000000";

struct MoovIoAchAdvEntryDetail {
    julian_day: i32,
    // ... other fields ...
}

impl MoovIoAchAdvEntryDetail {
    fn numeric_field(&self, max: u32) -> String {
        let s = self.julian_day.to_string();
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

struct MoovIoAchConverters;

fn get_padding(len: usize) -> String {
    ZEROS[..len].iter().cloned().collect::<Vec<u8>>().into_iter()
        .map(|c| c as char)
        .collect()
}

fn populate_map(max: i32, zero: char) -> HashMap<i32, String> {
    let mut map = HashMap::new();
    for i in 0..max {
        let key = i;
        let value = (0..i).map(|_| zero).collect::<String>();
        map.insert(key, value);
    }
    map
}

impl fmt::Display for MoovIoAchAdvEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // ... format and write the fields ...
        write!(f, "{}", self.numeric_field(3))
    }
}

