
use std::str::FromStr;

fn read_uint(s: &str) -> (i64, i64) {
    let i = s.find(|c| c == ' ' || c == '\t' || c == '\n').unwrap_or(s.len());
    let x = i64::from_str(&s[..i]).unwrap();
    (x, i as i64)
}

fn end(s: &str) -> i64 {
    for i in 0..s.len() as i64 {
        if s.chars().nth(i as usize).unwrap() == ' ' || s.chars().nth(i as usize).unwrap() == '\t' || s.chars().nth(i as usize).unwrap() == '\n' {
            return i;
        }
    }
    0
}
