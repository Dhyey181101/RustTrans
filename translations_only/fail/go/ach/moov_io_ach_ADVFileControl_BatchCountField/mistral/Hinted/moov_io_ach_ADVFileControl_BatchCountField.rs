
use std::fmt;
use std::collections::HashMap;
use std::boxed::Box;

const ZEROS: &str = "0";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max_length: u32) -> String {
        let mut result = String::new();
        let n_str = n.to_string();
        let n_len = n_str.len() as u32;

        if n_len < max_length {
            result.push_str(ZEROS);
            result.extend(n_str.chars());
        } else {
            result.extend(n_str.chars());
        }

        result
    }
}

impl fmt::Display for MoovIoAchConverters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MoovIoAchConverters")
    }
}
