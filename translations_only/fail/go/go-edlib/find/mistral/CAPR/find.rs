

use std::boxed;

type Runes = boxed::Box<[u32]>;

fn find(slice: Vec<Runes>, val: Runes) -> isize {
for (i, item) in slice.into_iter().enumerate() {
if equal(&item, &val) {
return i as isize;
}
}
-1
}

fn equal(a: &Runes, b: &Runes) -> bool {
if a.len() != b.len() {
return false;
}
for (i, v) in a.iter().enumerate() {
if v != &b[i] {
return false;
}
}
true
}

