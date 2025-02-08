

use std::fmt;
use std::collections::HashMap;
use std::boxed::Box;

const ZEROS: &str = "0";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            String::from(&s[s.len()-max as usize..])
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
        let mut v = String::new();
        for _ in 0..i {
            v.push_str(zero);
        }
        out.insert(i, v);
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
        let c = Box::new(MoovIoAchConverters);
        c.numeric_field(self.batch_count, 6)
    }
}

impl fmt::Display for MoovIoAchAdvFileControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ batch_count: {} }}", self.batch_count)
    }
}

fn main() {
    let mut map = populate_map(94, ZEROS);
    let c = Box::new(MoovIoAchConverters);
    let mut a = MoovIoAchAdvFileControl { batch_count: 123 };
    println!("{}", a.batch_count_field());
}

