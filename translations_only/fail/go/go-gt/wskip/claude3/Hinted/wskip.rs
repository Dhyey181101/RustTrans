
fn wskip(s: &str) -> &str {
    for (i, c) in s.char_indices() {
        if c != ' ' && c != '\t' {
            return &s[i..];
        }
    }
    ""
}
