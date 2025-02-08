

use std::fmt;
use std::collections::HashMap;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            s[s.len() - max as usize..].to_string()
        } else {
            let m = max as i32 - s.len() as i32;
            let pad = get_pad_string(m as usize);
            pad + &s
        }
    }
}

struct MoovIoAchIatEntryDetail {
    addenda_records: i32,
}

impl MoovIoAchIatEntryDetail {
    fn addenda_records_field(&self) -> String {
        MoovIoAchConverters.numeric_field(self.addenda_records, 4)
    }
}

fn get_pad_string(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, "0".repeat(i));
    }
    out[&n].clone()
}

impl fmt::Display for MoovIoAchIatEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AddendaRecords: {}",
            self.addenda_records_field()
        )
    }
}

fn main() {
    let iat_ed = Box::new(MoovIoAchIatEntryDetail {
        addenda_records: 123,
    });
    println!("{}", iat_ed);
}

