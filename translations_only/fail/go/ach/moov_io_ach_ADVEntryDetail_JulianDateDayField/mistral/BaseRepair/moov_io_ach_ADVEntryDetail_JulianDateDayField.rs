

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &str = "0";

struct MoovIoAchAdvEntryDetail {
    julian_day: i32,
    // ... other fields ...
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s[(l-max) as usize..].to_string()
        } else {
            let m = max - l;
            let pad = get_zeros(m as usize);
            format!("{}{}", pad, s)
        }
    }
}

fn get_zeros(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, "0".repeat(i));
    }
    (*out.get(&n).unwrap()).to_string()
}

impl MoovIoAchAdvEntryDetail {
    fn julian_date_day_field(&self) -> String {
        let converters = Box::new(MoovIoAchConverters);
        converters.numeric_field(self.julian_day, 3)
    }
}

impl fmt::Display for MoovIoAchAdvEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "JulianDay: {}, ",
            self.julian_date_day_field()
        )
    }
}

fn main() {
    let adv_entry = MoovIoAchAdvEntryDetail {
        julian_day: 20230315,
        // ... other fields ...
    };
    println!("{}", adv_entry);
}

