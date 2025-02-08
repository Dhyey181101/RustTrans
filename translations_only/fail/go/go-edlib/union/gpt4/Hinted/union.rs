
use std::collections::HashSet;

fn union(a: Vec<String>, b: Vec<String>) -> Vec<Vec<char>> {
    let mut set: HashSet<String> = HashSet::new();
    let mut combined = a.clone();

    for item in a.iter() {
        set.insert(item.clone());
    }

    for item in b.iter() {
        if !set.contains(item) {
            combined.push(item.clone());
        }
    }

    let mut out: Vec<Vec<char>> = Vec::new();
    for word in combined.iter() {
        out.push(word.chars().collect());
    }

    out
}

fn main() {}
