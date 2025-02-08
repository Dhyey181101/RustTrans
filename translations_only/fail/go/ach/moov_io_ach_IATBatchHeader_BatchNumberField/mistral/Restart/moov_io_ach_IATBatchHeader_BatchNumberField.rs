
use std::collections::HashMap;
use std::fmt;
use std::str;

const MAX_INT: i32 = 94;
const ZERO: &str = "0";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l < max {
            let mut missing = max - l;
            let mut result = String::new();
            while missing > 0 {
                result.push_str(ZERO);
                missing -= 1;
            }
            result.push_str(&s);
            return result;
        }
        s
    }
}

impl fmt::Display for MoovIoAchConverters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MoovIoAchConverters")
    }
}
