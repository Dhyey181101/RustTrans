

use std::mem;

struct Heap {
n: i64,
i: Vec<usize>,
a: Vec<i64>,
w: Vec<i64>,
}

impl Heap {
fn pop(&mut self) -> i64 {
let i = self.i[0];
self.n -= 1;
self.swap(0, self.n as usize);
self.down(0, self.n as usize);
i as i64
}

fn swap(&mut self, a: usize, b: usize) {
let temp_i = self.i[a];
self.i[a] = self.i[b];
self.i[b] = temp_i;

let temp_a = self.a[temp_i];
self.a[temp_i] = self.a[self.i[a]];
self.a[self.i[a]] = temp_a;
}

fn down(&mut self, mut i: usize, n: usize) {
let mut j: usize;
loop {
let left = 2 * i + 1;
if left >= n {
break;
}
j = left;
let right = left + 1;
if right < n && !self.less(left as i64, right as i64) {
j = right;
}
if self.less(i as i64, j as i64) {
break;
}
self.swap(i, j);
i = j;
}
}

fn less(&self, a: i64, b: i64) -> bool {
let i = self.i[a as usize];
let j = self.i[b as usize];
self.w[i] < self.w[j]
}
}

