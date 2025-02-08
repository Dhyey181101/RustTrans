
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Addenda17 {
    pub entry_detail_sequence_number: i32,
}

impl Addenda17 {
    pub fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[s.len() - max as usize..].to_string();
        } else {
            let m = max - s.len() as u32;
            let pad = STRING_ZEROS.get(&m).unwrap();
            return format!("{}{}", pad, s);
        }
    }
}

lazy_static::lazy_static! {
    static ref STRING_ZEROS: HashMap<u32, String> = populate_map(94, "0");
}

fn populate_map(max: i32, zero: &str) -> HashMap<u32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i as u32, zero.repeat(i as usize));
    }
    out
}

impl fmt::Display for Addenda17 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Addenda17 {{ entry_detail_sequence_number: {} }}",
            self.entry_detail_sequence_number
        )
    }
}
