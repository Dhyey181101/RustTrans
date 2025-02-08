
fn end(s: &str) -> i64 {
    for i in 0..s.len() as i64 {
        if s.chars().nth(i as usize).unwrap() == ' ' || s.chars().nth(i as usize).unwrap() == '\t' || s.chars().nth(i as usize).unwrap() == '\n' {
            return i;
        }
    }
    return 0;
}
