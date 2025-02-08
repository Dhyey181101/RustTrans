
use std::str::FromStr;

fn read_uint(s: &str) -> (i64, usize) {
    let i = end(s);
    let x = i64::from_str(&s[..i]).unwrap_or(0);
    (x, i) 
}

fn end(s: &str) -> usize {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' || byte == b'\t' || byte == b'\n' {
            return i;
        }
    }
    0
}
