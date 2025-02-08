
use std::collections::HashMap;

#[derive(Debug)]
pub struct Addenda05 {
    pub sequence_number: i32,
}

impl Addenda05 {
    pub fn sequence_number_field(&self) -> String {
        let converters = Converters {};
        converters.numeric_field(self.sequence_number, 4)
    }
}

pub struct Converters {}

impl Converters {
    pub fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() - max as usize)..].to_string();
        } else {
            let m = max - s.len() as u32;
            let pad = string_zeros(m as usize);
            return format!("{}{}", pad, s);
        }
    }
}

fn string_zeros(m: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..m {
        out.insert(i, "0".repeat(i));
    }
    out[&m].clone()
}
