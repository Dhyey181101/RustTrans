
use std::str;

fn end(s: &str) -> usize {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        match byte {
            b' ' | b'\t' | b'\n' => return i,
            _ => {}
        }
    }
    0
}
