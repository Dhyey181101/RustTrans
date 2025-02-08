
fn read_uint(s: &str) -> (i64, usize) {
    let i = end(s);
    let x = s[..i].parse::<i64>().unwrap();
    (x, i)
}

fn end(s: &str) -> usize {
    for i in 0..s.len() {
        if s.chars().nth(i).unwrap() == ' ' || s.chars().nth(i).unwrap() == '\t' || s.chars().nth(i).unwrap() == '\n' {
            return i;
        }
    }
    0
}
