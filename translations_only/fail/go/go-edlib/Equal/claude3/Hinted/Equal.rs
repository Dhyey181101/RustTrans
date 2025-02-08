
fn equal(a: &[u32], b: &[u32]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for (i, v) in a.iter().enumerate() {
        if *v != b[i] {
            return false;
        }
    }
    true
}
