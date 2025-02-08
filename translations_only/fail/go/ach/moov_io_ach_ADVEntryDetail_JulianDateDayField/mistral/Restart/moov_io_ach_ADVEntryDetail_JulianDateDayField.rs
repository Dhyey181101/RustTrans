

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
            padding.chars().chain(s.chars()).collect()
        }
    }
}

struct MoovIoAchConverters;

fn get_padding(len: usize) -> String {
    ZEROS[..len].iter()
        .map(|c| (*c as char).to_string())
        .collect()
}

fn populate_map(max: i32, zero: char) -> HashMap<i32, String> {
    let mut map = HashMap::new();
    for i in 0..max {
        let key = i;
        let value = (0..i)
            .map(|_| zero)
            .collect::<String>();
        map.insert(key, value);
    }
    map
}

impl fmt::Display for MoovIoAchAdvEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "JulianDay: {}, ",
            self.julian_day
        )
    }
}

fn main() {
    let mut map = populate_map(94, '0');
    let entry = Box::new(MoovIoAchAdvEntryDetail {
        julian_day: 123,
        // ... other fields ...
    });

    println!("{}", entry.numeric_field(3));
}

