
use std::collections::HashMap;
use std::fmt;

const ZEROS: &str = "0";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l < max {
            let mut missing_zeros = max - l;
            let mut result = String::new();
            while missing_zeros > 0 {
                result.push_str(ZEROS);
                missing_zeros -= 1;
            }
            result.push_str(&s);
            return result;
        }
        s
    }
}

fn main() {
    let m = MoovIoAchConverters;
    let n = m.numeric_field(1, 4);
    println!("{}", n);
}
