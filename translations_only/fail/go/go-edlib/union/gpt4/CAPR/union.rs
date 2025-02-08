
use std::collections::HashSet;

fn union(a: Vec<String>, b: Vec<String>) -> Vec<Vec<char>> {
    let mut set: HashSet<String> = HashSet::new();
    let mut result: Vec<Vec<char>> = Vec::new();

    for item in a.iter().chain(b.iter()) {
        if set.insert(item.clone()) {
            result.push(item.chars().collect());
        }
    }

    result
}

fn main() {}
