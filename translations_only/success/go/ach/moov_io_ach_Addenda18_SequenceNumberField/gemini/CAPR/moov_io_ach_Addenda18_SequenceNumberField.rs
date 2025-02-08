
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct Addenda18 {
    pub sequence_number: i32,
    pub entry_detail_sequence_number: i32,
}

impl Addenda18 {
    pub fn sequence_number_field(&self) -> String {
        self.numeric_field(self.sequence_number, 4)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() - max as usize)..].to_string();
        } else {
            let m = max - s.len() as u32;
            let pad = STRING_ZEROS.get(&m).unwrap();
            return format!("{}{}", pad, s);
        }
    }
}

lazy_static! {
    static ref STRING_ZEROS: HashMap<u32, String> = populate_map(94, "0");
}

fn populate_map(max: i32, zero: &str) -> HashMap<u32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i as u32, zero.repeat(i as usize));
    }
    out
}
