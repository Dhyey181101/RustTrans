
pub fn wskip(s: &str) -> &str {
    for i in 0..s.len() {
        if s.chars().nth(i).unwrap() != ' ' && s.chars().nth(i).unwrap() != '\t' {
            return &s[i..];
        }
    }
    ""
}
