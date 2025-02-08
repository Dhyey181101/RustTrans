
use std::collections::HashSet;

fn union(a: Vec<String>, b: Vec<String>) -> Vec<Vec<char>> {
    let mut set: HashSet<String> = HashSet::new();
    for item in a {
        set.insert(item);
    }
    for item in b {
        set.insert(item);
    }

    let mut out: Vec<Vec<char>> = Vec::with_capacity(set.len());
    for word in set {
        out.push(word.chars().collect());
    }
    out
}
