

use std::fmt;
use std::collections::HashMap;
use std::str;

const ZEROS: &str = "0";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s[l as usize - max as usize..].to_string()
        } else {
            let m = max - l;
            let pad = get_pad(m as usize);
            pad + &s
        }
    }
}

fn get_pad(n: usize) -> String {
    let mut out = String::new();
    for _ in 0..n {
        out.push('0');
    }
    out
}

fn populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        let mut s = String::new();
        for _ in 0..i {
            s.push_str(zero);
        }
        out.insert(i, s);
    }
    out
}

#[derive(Default)]
struct MoovIoAchAdvFileControl {
    batch_count: i32,
    // other fields elided for brevity
}

impl MoovIoAchAdvFileControl {
    fn batch_count_field(&self) -> String {
        MoovIoAchConverters::new().numeric_field(self.batch_count, 6)
    }
}

impl fmt::Display for MoovIoAchAdvFileControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ batch_count: {} }}", self.batch_count)
    }
}

impl MoovIoAchConverters {
    fn new() -> MoovIoAchConverters {
        MoovIoAchConverters {}
    }
}

fn main() {
    let mut map = populate_map(94, ZEROS);
    let c = MoovIoAchConverters::new();
    let mut a = MoovIoAchAdvFileControl { batch_count: 123 };
    println!("{}", a.batch_count_field());
}

