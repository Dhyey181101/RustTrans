

use std::collections::HashMap;
use std::fmt;

const ZEROS: &str = "0";

struct EntryDetail {
    trace_number: String,
}

struct Converters;

impl EntryDetail {
    fn trace_number_field(&self) -> String {
        let conv = Box::new(Converters);
        conv.string_field(self.trace_number.clone(), 15)
    }
}

impl Converters {
    fn string_field(&self, s: String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = (max - ln) as usize;
            let pad = get_zeros(m);
            pad + &s
        }
    }
}

fn get_zeros(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, ZEROS.to_string().repeat(i));
    }
    out[&n].clone()
}

impl fmt::Display for EntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.trace_number_field())
    }
}

fn main() {
    let ed = EntryDetail {
        trace_number: "123456789012345".to_string(),
    };
    println!("{}", ed);
}

