

use std::fmt;
use std::collections::HashMap;
use std::str;

const ZEROS: &str = "0";

struct MoovIoAchAdvEntryDetail {
    sequence_number: i32,
}

struct MoovIoAchConverters;

impl MoovIoAchAdvEntryDetail {
    fn sequence_number_field(&self) -> String {
        let n = self.sequence_number;
        let s = n.to_string();
        let l = s.len() as u32;
        let padding = (4 - l) as usize;
        let mut padded_string = String::from("");
        if padding > 0 {
            padded_string.push_str(&ZEROS[..padding]);
        }
        padded_string.push_str(&s);
        padded_string
    }
}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        let padding = (max - l) as usize;
        let mut padded_string = String::from("");
        if padding > 0 {
            padded_string.push_str(&ZEROS[..padding]);
        }
        padded_string.push_str(&s);
        padded_string
    }
}

fn main() {
    let v = Box::new(10);
}

