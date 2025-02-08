

use std::fmt;
use std::collections::HashMap;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = get_pad_string(m as usize);
            return pad + &s;
        }
    }
}

fn get_pad_string(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, "0".repeat(i));
    }
    out[&n].clone()
}

struct MoovIoAchIATEntryDetail {
    addenda_records: i32,
}

impl MoovIoAchIATEntryDetail {
    fn addenda_records_field(&self) -> String {
        MoovIoAchConverters.numeric_field(self.addenda_records, 4)
    }
}

fn main() {
    let iat_ed = Box::new(MoovIoAchIATEntryDetail { addenda_records: 123 });
    println!("{}", iat_ed.addenda_records_field());
}

