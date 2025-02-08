
fn wskip(s: &str) -> &str {
    for (i, c) in s.chars().enumerate() {
        if c != ' ' && c != '\t' {
            return &s[i..];
        }
    }
    ""
}
