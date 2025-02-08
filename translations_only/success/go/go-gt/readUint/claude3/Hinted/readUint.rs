
use std::str::from_utf8;

fn read_uint(s: &str) -> (i64, usize) {
    let end_index = end(s);
    let x = from_utf8(&s.as_bytes()[..end_index])
        .unwrap()
        .parse::<i64>()
        .unwrap_or(0);
    (x, end_index)
}

fn end(s: &str) -> usize {
    for (i, &c) in s.as_bytes().iter().enumerate() {
        if c == b' ' || c == b'\t' || c == b'\n' {
            return i;
        }
    }
    0
}
