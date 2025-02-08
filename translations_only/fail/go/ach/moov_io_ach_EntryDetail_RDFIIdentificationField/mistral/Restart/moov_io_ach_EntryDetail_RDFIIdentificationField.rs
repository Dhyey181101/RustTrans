

use std::collections::HashMap;
use std::fmt;

const ZEROS: &str = "0";

struct EntryDetail {
    rdfi_identification: String,
}

struct Converters;

impl EntryDetail {
    fn rdfi_identification_field(&self) -> String {
        Converters::string_field(&self.rdfi_identification, 8)
    }
}

impl Converters {
    fn string_field(s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        let pad = get_zeros(m);
        pad.chars().chain(s.chars()).collect()
    }
}

fn get_zeros(n: usize) -> String {
    let mut zeros = HashMap::new();
    populate_map(&mut zeros, 94, ZEROS);
    zeros.get(&n).cloned().unwrap_or(String::from(""))
}

fn populate_map(map: &mut HashMap<usize, String>, max: usize, zero: &str) {
    for i in 0..=max {
        let mut buf: String = "".to_string();
        for _ in 0..i {
            buf.push_str(zero);
        }
        map.insert(i, buf);
    }
}

impl fmt::Display for EntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.rdfi_identification_field())
    }
}

