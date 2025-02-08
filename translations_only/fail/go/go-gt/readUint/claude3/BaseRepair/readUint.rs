
use std::str::FromStr;

fn read_uint(s: &str) -> (i64, usize) {
    let i = end(s);
    let x = i64::from_str(&s[..i]).unwrap();
    (x, i)
}

fn end(s: &str) -> usize {
    for (i, c) in s.char_indices() {
        if c == ' ' || c == '\t' || c == '\n' {
            return i;
        }
    }
    0
}
