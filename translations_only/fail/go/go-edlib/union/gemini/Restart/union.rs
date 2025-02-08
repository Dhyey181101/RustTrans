
use std::collections::HashMap;

pub fn union(mut a: Vec<String>, b: Vec<String>) -> Vec<Vec<char>> {
    let mut m: HashMap<String, bool> = HashMap::new();
    for item in a.iter() {
        m.insert(item.clone(), true);
    }
    for item in b.iter() {
        if !m.contains_key(item) {
            a.push(item.clone());
        }
    }

    a.iter().map(|word| word.chars().collect()).collect()
}
