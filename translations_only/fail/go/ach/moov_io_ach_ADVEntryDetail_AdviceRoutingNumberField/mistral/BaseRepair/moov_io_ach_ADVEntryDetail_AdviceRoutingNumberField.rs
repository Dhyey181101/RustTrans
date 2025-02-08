

use std::collections::HashMap;
use std::fmt;

const ZEROS: &[u8] = b"0000000000000000000000000000000000000000000000000000000000000000";

struct MoovIoAchAdvEntryDetail {
    advice_routing_number: String,
}

impl MoovIoAchAdvEntryDetail {
    fn advice_routing_number_field(&self) -> String {
        MoovIoAchConverters::string_field(&self.advice_routing_number, 9)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(s: &str, max: usize) -> String {
        let len = s.len();
        if len > max {
            s[..max].to_string()
        } else {
            let padding_len = max - len;
            let padding = get_padding(padding_len);
            padding.to_string() + s
        }
    }
}

fn get_padding(n: usize) -> String {
    let padding = ZEROS[..n].to_vec();
    unsafe { String::from_utf8_unchecked(padding) }
}

fn populate_map(max: usize, zero: &[u8]) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        let value = get_padding(i);
        out.insert(i, value);
    }
    out
}

#[derive(Debug)]
struct MyString(String);

impl fmt::Display for MyString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    let mut map = populate_map(100, ZEROS);
    let detail = MoovIoAchAdvEntryDetail {
        advice_routing_number: "123456789".to_string(),
    };
    println!("{}", detail.advice_routing_number_field());
}

