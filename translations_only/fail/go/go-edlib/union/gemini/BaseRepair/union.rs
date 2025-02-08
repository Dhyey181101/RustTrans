
use std::collections::HashMap;

pub fn union(a: &[String], b: &[String]) -> Vec<Vec<char>> {
    let mut m: HashMap<String, bool> = HashMap::new();
    for item in a {
        m.insert(item.clone(), true);
    }
    for item in b {
        if !m.contains_key(item) {
            m.insert(item.clone(), true);
        }
    }

    m.keys().map(|word| word.chars().collect()).collect()
}
