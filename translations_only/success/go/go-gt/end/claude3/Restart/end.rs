
fn end(s: &str) -> u64 {
    for (i, c) in s.char_indices() {
        if c == ' ' || c == '\t' || c == '\n' {
            return i as u64;
        }
    }
    0
}
