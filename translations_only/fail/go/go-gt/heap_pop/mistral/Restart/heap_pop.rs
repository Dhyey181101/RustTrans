

use std::mem;

struct Heap {
n: i64,
i: Vec<i64>,
a: Vec<i64>,
w: Vec<i64>,
}

impl Heap {
fn pop(&mut self) -> i64 {
let node = self.i.pop().unwrap();
let mut val = self.a[node as usize];
let mut child = self.i[node as usize] as usize;
while child < self.i.len() {
let mut next_child = child + 1;
if next_child < self.i.len() && self.w[self.i[next_child as usize] as usize] < self.w[self.i[child as usize] as usize] {
next_child += 1;
}
val = val.min(self.a[self.i[next_child as usize] as usize]);
self.i[child as usize] = self.i[next_child as usize];
child = next_child;
}
self.a[node as usize] = mem::replace(&mut self.a[child as usize], val);
child as i64
}
}

