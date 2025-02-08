
use std::fmt;
use std::collections::HashMap;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max {
            s[l-max..].to_string()
        } else {
            let m = max - l;
            let pad = get_pad_string(m);
            pad + &s
        }
    }
}

fn get_pad_string(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..n {
        out.insert(i, "0".repeat(i));
    }
    out[&n].clone()
}

struct MoovIoAchAddenda17 {
    sequence_number: i32,
    // other fields omitted for brevity
}

impl MoovIoAchAddenda17 {
    fn new(sequence_number: i32) -> MoovIoAchAddenda17 {
        MoovIoAchAddenda17 {
            sequence_number,
            // initialize other fields here
        }
    }
}

impl fmt::Display for MoovIoAchAddenda17 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{:0>15}", "17", self.sequence_number)
    }
}
