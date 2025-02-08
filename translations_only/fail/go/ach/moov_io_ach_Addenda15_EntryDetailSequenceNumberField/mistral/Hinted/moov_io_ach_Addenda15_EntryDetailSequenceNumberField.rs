

use std::fmt;
use std::collections::HashMap;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        if n > (max as i32) {
            format!("{} (overflow)", n)
        } else if n < -(max as i32) {
            format!("{} (underflow)", n)
        } else {
            n.to_string()
        }
    }
}

impl fmt::Display for MoovIoAchConverters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MoovIoAchConverters")
    }
}

fn main() {
    let m = MoovIoAchConverters {};
    let n = m.numeric_field(10, 5);
    println!("{}", n);
}

