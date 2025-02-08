
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Addenda18 {
    pub entry_detail_sequence_number: i32,
}

impl Addenda18 {
    pub fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() - max as usize)..].to_string();
        } else {
            let m = max as usize - s.len();
            let pad = string_zeros.get(&m).unwrap();
            return format!("{}{}", pad, s);
        }
    }
}

lazy_static! {
    static ref string_zeros: HashMap<usize, String> = populate_map(94, "0");
}

fn populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}
