

use std::mem;

struct Heap {
n: i64,
i: Vec<usize>,
a: Vec<Box<i64>>,
w: Vec<i64>,
}

fn less(heap: &Heap, a: usize, b: usize) -> bool {
heap.w[a] < heap.w[b]
}

fn down(heap: &mut Heap, mut i: i64) {
let mut j = 2 * i + 1;
while j < heap.n {
let right = j + 1;
if right < heap.n && less(heap, heap.i[right as usize], heap.i[j as usize]) {
j = right;
}
if less(heap, heap.i[i as usize], heap.i[j as usize]) {
let temp_i = heap.i[i as usize];
let temp_a = heap.a[heap.i[i as usize]].clone();
heap.i[i as usize] = heap.i[j as usize];
heap.a[heap.i[i as usize]] = heap.a[heap.i[j as usize]].clone();
heap.i[j as usize] = temp_i;
heap.a[heap.i[j as usize]] = temp_a;
}
i = j;
j = 2 * i + 1;
}
}

fn swap(heap: &mut Heap, a: i64, b: i64) {
let temp_i = heap.i[a as usize];
heap.i[a as usize] = heap.i[b as usize];
heap.i[b as usize] = temp_i;
}

