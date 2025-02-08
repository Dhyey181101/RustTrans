
fn union(mut a: Vec<String>, b: Vec<String>) -> Vec<Vec<char>> {
    let mut m = std::collections::HashSet::new();
    for item in &a {
        m.insert(item.to_string());
    }
    for item in &b {
        if !m.contains(item) {
            a.push(item.to_string());
        }
    }

    let mut out = Vec::with_capacity(a.len());
    for word in a {
        out.push(word.chars().collect());
    }
    out
}
