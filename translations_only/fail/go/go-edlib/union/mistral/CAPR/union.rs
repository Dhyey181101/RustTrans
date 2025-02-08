

use std::boxed::Box;
use std::collections::HashMap;

fn union(mut a: Vec<String>, b: Vec<String>) -> Vec<Vec<u8>> {
    let mut m = HashMap::new();
    for item in &a {
        m.insert(item.clone(), true);
    }
    for item in &b {
        if m.get(item).is_none() {
            a.push(item.clone());
        }
    }

    let mut out = vec![];
    for word in a {
        out.push(word.chars().map(|c| c as u8).collect::<Vec<u8>>());
    }
    out
}

