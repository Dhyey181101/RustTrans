
use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;

struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn format_simple_date(&self, s: &str) -> String {
        if s.is_empty() {
            return self.string_field(s, 6);
        }
        s.to_string()
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let binding = moov_io_ach_string_zeros();
        let pad = binding.get(&m).unwrap();
        format!("{}{}", pad, s)
    }
}

fn moov_io_ach_string_zeros() -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..94 {
        out.insert(i, String::from_str("0").unwrap().repeat(i));
    }
    out
}

fn main() {
    let c = MoovIoAchConverters {};
    let s = c.format_simple_date("20220308");
    println!("{}", s);
    let s = c.string_field("12345", 6);
    println!("{}", s);
}
