
use std::str;

fn end(s: &str) -> usize {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' || byte == b'\t' || byte == b'\n' {
            return i;
        }
    }
    return 0;
}
