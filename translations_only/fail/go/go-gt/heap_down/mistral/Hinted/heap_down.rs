

use std::mem;

struct Heap {
n: i64,
i: Vec<i64>,
a: Vec<Box<i64>>,
w: Vec<i64>,
}

fn less(heap: &Heap, a: i64, b: i64) -> bool {
heap.w[a as usize] < heap.w[b as usize]
}

fn new_heap(n: i64) -> Heap {
Heap {
n,
i: Vec::new(),
a: vec![Box::new(0); n as usize],
w: Vec::new(),
}
}

fn push_heap(heap: &mut Heap, x: i64) {
let n = heap.n;
heap.n += 1;
let mut i = (heap.n - 1) as usize;
let mut p = (i - 1) / 2;
while i != 0 && less(heap, i as i64, p as i64) {
let temp = heap.a[i].clone();
heap.a[i] = heap.a[p].clone();
heap.a[p] = temp;
i = p;
p = (i - 1) / 2;
}
heap.i.push(x);
heap.w.push(0);
}

fn adjust_heap(heap: &mut Heap, i: i64) {
let mut i = i as usize;
let n = (heap.n - 1) as usize;
loop {
let l = 2 * i + 1;
let r = 2 * i + 2;
let mut largest = if l < n && less(heap, l as i64, r as i64) { r } else { l };
if r < n && less(heap, r as i64, largest as i64) {
largest = r;
}
if largest == i {
break;
}
let temp = heap.a[i].clone();
heap.a[i] = heap.a[largest].clone();
heap.a[largest] = temp;
i = largest;
}
}

fn pop_heap(heap: &mut Heap) -> i64 {
let n = (heap.n - 1) as usize;
let temp = heap.a[n].clone();
heap.a[n] = heap.a[0].clone();
heap.a[0] = temp;
heap.n = n as i64;
let result = *heap.a.pop().unwrap();
heap.i.pop();
heap.w.pop();
adjust_heap(heap, 0);
result
}

