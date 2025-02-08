
use std::str::FromStr;

fn read_uint(s: &str) -> (i64, usize) {
    let i = end(s);
    let x = i64::from_str(&s[..i]).unwrap();
    (x, i)
}

fn end(s: &str) -> usize {
    for i in 0..s.len() {
        if s.chars().nth(i).unwrap().is_whitespace() {
            return i;
        }
    }
    0
}
