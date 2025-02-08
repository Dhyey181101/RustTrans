

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
            let pad = MoovIoAchIatEntryDetail::get_pad_string(m);
            return pad + &s;
        }
    }
}

struct MoovIoAchIatEntryDetail {
    addenda_records: i32,
}

impl MoovIoAchIatEntryDetail {
    fn addenda_records_field(&self) -> String {
        self.numeric_field(self.addenda_records, 4)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = MoovIoAchIatEntryDetail::get_pad_string(m);
            return pad + &s;
        }
    }

    fn get_pad_string(n: u32) -> String {
        let mut map = HashMap::new();
        for i in 0..94 {
            let zero = "0".repeat(i as usize);
            map.insert(i, zero);
        }
        let pad = map.get(&n).unwrap_or(&"0".repeat(n as usize)).to_string();
        pad
    }
}

fn main() {
    let iat_ed = MoovIoAchIatEntryDetail { addenda_records: 1234 };
    println!("Addenda Records Field: {}", iat_ed.addenda_records_field());
}

