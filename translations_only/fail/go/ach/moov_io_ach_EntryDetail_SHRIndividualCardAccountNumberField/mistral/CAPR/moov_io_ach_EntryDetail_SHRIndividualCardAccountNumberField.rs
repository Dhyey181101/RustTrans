

use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';
const CHECKING_CREDIT: u32 = 22;

struct EntryDetail {
    individual_name: String,
    // ... other fields elided ...
    converters: Box<Converters>,
}

struct Converters {
    string_field_max: u32,
}

impl EntryDetail {
    fn shr_individual_card_account_number_field(&self) -> String {
        self.converters.string_field(&self.individual_name, CHECKING_CREDIT)
    }
}

impl Converters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let ln: usize = s.chars().count();
        let max = max as usize;
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = get_padding_string(m);
        format!("{}{}", pad, s)
    }
}

fn get_padding_string(n: usize) -> String {
    let mut map = HashMap::new();
    for i in 0..n {
        map.insert(i, "0".repeat(i));
    }
    String::from(&map[&n])
}

impl fmt::Display for EntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Implementation elided
        unimplemented!()
    }
}

