
use std::str;

fn read_uint(s: &str) -> (i64, usize) {
    let i = end(s);
    let x: i64 = match s[..i].parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    (x, i)
}

fn end(s: &str) -> usize {
    s.chars().position(|c| c == ' ' || c == '\t' || c == '\n').unwrap_or(s.len())
}
