
fn end(s: &str) -> i64 {
    let mut i = 0;
    let len = s.len() as i64;
    while i < len {
        if s.as_bytes()[i as usize] == b' ' || s.as_bytes()[i as usize] == b'\t' || s.as_bytes()[i as usize] == b'\n' {
            return i;
        }
        i += 1;
    }
    0
}
