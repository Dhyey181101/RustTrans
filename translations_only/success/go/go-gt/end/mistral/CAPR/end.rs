
use std::str;

fn end(s: &str) -> i64 {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        match byte {
            b' ' | b'\t' | b'\n' => return i as i64,
            _ => {}
        }
    }
    0
}
