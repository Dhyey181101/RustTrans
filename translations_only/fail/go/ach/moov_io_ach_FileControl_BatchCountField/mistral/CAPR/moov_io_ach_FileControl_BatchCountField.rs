

use std::fmt;
use std::collections::HashMap;
use std::boxed::Box;
use std::iter; // added

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            String::from(&s[l as usize-max as usize..])
        } else {
            let m = max - l;
            let pad = get_pad_string(m as usize);
            pad + &s
        }
    }
}

fn get_pad_string(n: usize) -> String {
    iter::repeat("0").take(n).collect::<String>()
}

struct MoovIoAchFileControl {
    batch_count: i32,
    // other fields elided for brevity
}

impl MoovIoAchFileControl {
    fn batch_count_field(&self) -> String {
        let m = Box::new(MoovIoAchConverters);
        m.numeric_field(self.batch_count, 6)
    }
}

fn main() {
    let fc = MoovIoAchFileControl {
        batch_count: 123,
        // other fields elided for brevity
    };
    println!("{}", fc.batch_count_field());
}

