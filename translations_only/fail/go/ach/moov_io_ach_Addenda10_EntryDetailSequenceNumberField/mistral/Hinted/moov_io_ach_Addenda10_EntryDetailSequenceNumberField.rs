

use std::collections::HashMap;
use std::fmt;
use std::iter;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            s[s.len() - max as usize..].to_string()
        } else {
            let m = max as i32 - s.len() as i32;
            let pad = get_zeros(m as usize);
            if let Some(p) = pad {
                p + &s
            } else {
                iter::repeat('0').take(m as usize).collect::<String>() + &s
            }
        }
    }
}

fn get_zeros(n: usize) -> Option<String> {
    let mut map = HashMap::new();
    for i in 0..=n {
        map.insert(i, "0".repeat(i));
    }
    map.get(&n).cloned()
}

struct MoovIoAchAddenda10 {
    entry_detail_sequence_number: i32,
    _converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda10 {
    fn entry_detail_sequence_number_field(&self) -> String {
        self._converters.numeric_field(self.entry_detail_sequence_number, 7)
    }
}

fn main() {
    let _addenda10 = MoovIoAchAddenda10 {
        entry_detail_sequence_number: 123456,
        _converters: Box::new(MoovIoAchConverters),
    };

    println!("{}", _addenda10.entry_detail_sequence_number_field());
}

