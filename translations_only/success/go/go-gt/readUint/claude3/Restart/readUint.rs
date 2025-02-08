
use std::str::FromStr;

fn read_uint(s: &str) -> (i64, usize) {
    let end_idx = end(s);
    let x = i64::from_str(&s[..end_idx]).unwrap_or(0);
    (x, end_idx)
}

fn end(s: &str) -> usize {
    for (i, c) in s.char_indices() {
        if c == ' ' || c == '\t' || c == '\n' {
            return i;
        }
    }
    0
}
