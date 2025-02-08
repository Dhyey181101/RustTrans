
fn wskip(s: &str) -> &str {
    let mut i = 0;
    let len = s.len();
    while i < len && (s.as_bytes()[i] == b' ' || s.as_bytes()[i] == b'\t') {
        i += 1;
    }
    &s[i..]
}

