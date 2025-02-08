

use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';
const CHECKING_CREDIT: u8 = 22;

struct EntryDetail {
    individual_name: String,
    // ... other fields ...
    converters: Box<Converters>,
}

struct Converters;

impl EntryDetail {
    fn shr_individual_card_account_number_field(&self) -> String {
        self.converters.string_field(&self.individual_name, CHECKING_CREDIT as u32)
    }
}

impl Converters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let ln: usize = s.chars().count();
        let mut result = String::with_capacity(s.len());

        if ln > max as usize {
            result.push_str(&s[..(max as usize)]);
        } else {
            let m: usize = (max as usize - ln).into();
            result.extend(self.pad_string(m));
            result.push_str(s);
        }

        result
    }

    fn pad_string(&self, count: usize) -> Vec<char> {
        let mut result = Vec::new();
        for _ in 0..count {
            result.push(ZERO);
        }
        result
    }
}

fn main() {}

