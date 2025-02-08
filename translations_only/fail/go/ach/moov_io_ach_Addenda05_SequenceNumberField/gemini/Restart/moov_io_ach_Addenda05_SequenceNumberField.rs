
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Addenda05 {
    pub sequence_number: i32,
}

impl Addenda05 {
    pub fn sequence_number_field(&self) -> String {
        self.numeric_field(self.sequence_number, 4)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() - max as usize)..].to_string();
        } else {
            let m = (max - s.len() as u32) as usize;
            let pad = STRING_ZEROS.get(&m).unwrap();
            return format!("{}{}", pad, s);
        }
    }
}

lazy_static! {
    static ref STRING_ZEROS: HashMap<usize, String> = (0..94).map(|i| (i, "0".repeat(i))).collect();
}

fn main() {
    let addenda05 = Addenda05 { sequence_number: 1234 };
    println!("{}", addenda05.sequence_number_field());
}
