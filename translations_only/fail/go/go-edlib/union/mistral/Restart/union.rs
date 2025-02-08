

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
let v: Vec<u8> = word.into_bytes();
out.push(v);
}
out
}

