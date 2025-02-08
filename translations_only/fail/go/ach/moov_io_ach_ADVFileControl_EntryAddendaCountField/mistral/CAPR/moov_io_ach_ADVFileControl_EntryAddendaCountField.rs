
use std::fmt;
use std::collections::HashMap;
use std::boxed::Box;

const ZEROS: &str = "0";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l < max {
            let mut missing = max - l;
            let mut result = String::new();
            while missing > 0 {
                result.push_str(ZEROS);
                missing -= 1;
            }
            result.push_str(&s);
            return result;
        }
        s
    }
}

fn main() {
    let m = MoovIoAchConverters;
    println!("{}", m.numeric_field(123, 5));
}
