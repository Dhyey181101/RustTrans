

use std::collections::HashMap;
use once_cell::sync::Lazy;

fn populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

static STRING_ZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| populate_map(94, "0"));

struct Addenda18 {
    entry_detail_sequence_number: i64,
}

impl Addenda18 {
    fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number as i64, 7)
    }

    fn numeric_field(&self, n: i64, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            String::from(&s[(l - max) as usize..])
        } else {
            let m = (max - l) as usize;
            let pad = STRING_ZEROS.get(&(m as i32)).unwrap_or(&"0".repeat(m)).to_owned();
            pad + &s
        }
    }
}

struct Converters {}

impl Converters {
    fn new() -> Box<Converters> {
        Box::new(Converters {})
    }
}

