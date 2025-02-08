
fn end(s: &str) -> i64 {
    for (i, c) in s.chars().enumerate() {
        if c == ' ' || c == '\t' || c == '\n' {
            return i as i64;
        }
    }
    0
}
