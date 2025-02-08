

use std::mem;

struct Heap {
n: i64,
i: Vec<i64>,
a: Vec<i64>,
w: Vec<Box<i64>>,
}

impl Heap {
fn pop(&mut self) -> i64 {
let node = self.i.pop().unwrap();
let mut v = node;
let mut index = node as usize;
while index < self.a.len() {
let child = self.a[index];
if child == -1 {
break;
}
let child_value = *self.w[child as usize];
if *self.w[v as usize] < child_value {
v = child;
index = child as usize;
} else {
break;
}
}
if node != v {
self.i[v as usize] = node;
self.i[node as usize] = -1;
self.a[node as usize] = self.a[v as usize];
self.a[v as usize] = -1;
self.w[node as usize] = self.w[v as usize].clone();
}
let temp = self.w[node as usize].clone();
self.w[v as usize] = temp;
self.w[node as usize] = Box::new(0);
return *self.w[node as usize];
}
}

