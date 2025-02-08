
use std::collections::HashSet;

fn union(a: Vec<String>, b: Vec<String>) -> Vec<Vec<char>> {
    let mut set: HashSet<String> = a.iter().cloned().collect();
    let mut result = a;

    for item in b.into_iter() {
        if !set.contains(&item) {
            result.push(item);
        }
    }

    result.into_iter().map(|word| word.chars().collect()).collect()
}

fn main() {}
