
fn read_uint(s: &str) -> (i64, i64) {
    let i = end(s);
    let x = s[..i as usize].parse::<i64>().unwrap_or(0);
    (x, i)
}

fn end(s: &str) -> i64 {
    for (i, c) in s.chars().enumerate() {
        if c == ' ' || c == '\t' || c == '\n' {
            return i as i64;
        }
    }
    0
}
