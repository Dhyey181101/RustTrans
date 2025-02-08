
fn equal(a: &[char], b: &[char]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for (i, &v) in a.iter().enumerate() {
        if v != b[i] {
            return false;
        }
    }
    true
}
