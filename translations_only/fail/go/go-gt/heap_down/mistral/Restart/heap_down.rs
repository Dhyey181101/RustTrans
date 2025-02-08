

use std::mem;

struct Heap {
n: i64,
i: Vec<usize>,
a: Vec<Box<i64>>,
w: Vec<i64>,
}

fn down(heap: &mut Heap, mut i: i64) {
let mut j = 2 * i + 1;
while j < heap.n {
let right = j + 1;
if right < heap.n && !heap.less(j, right) {
j = right;
}
if !heap.less(i, j) {
break;
}
heap.swap(i, j);
i = j;
j = 2 * i + 1;
}
}

impl Heap {
fn less(&self, a: i64, b: i64) -> bool {
self.w[self.i[a as usize]] < self.w[self.i[b as usize]]
}

fn swap(&mut self, mut a: i64, mut b: i64) {
let temp_i = self.i[a as usize];
self.i[a as usize] = self.i[b as usize];
self.i[b as usize] = temp_i;

let temp_a = *self.a[self.i[a as usize]].clone();
*self.a[self.i[a as usize]] = *self.a[self.i[b as usize]].clone();
*self.a[self.i[b as usize]] = temp_a;
}
}

