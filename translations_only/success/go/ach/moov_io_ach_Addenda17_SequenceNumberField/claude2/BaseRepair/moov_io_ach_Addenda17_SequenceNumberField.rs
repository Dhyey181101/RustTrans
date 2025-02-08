

use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| HashMap::new());

struct Addenda17 {
    sequence_number: i32,
}

impl Addenda17 {
    fn sequence_number_field(&self) -> String {
        numeric_field(self.sequence_number, 4)
    }
}

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    if s.len() as u32 > max {
        s[s.len()-max as usize..].to_string()
    } else {
        let m = (max - s.len() as u32) as i32;
        let zero = "0".to_string();
        let mut res = String::new();
        for _ in 0..m {
            res.push_str(&zero);
        }
        res + &s
    }
}

struct Converters;

