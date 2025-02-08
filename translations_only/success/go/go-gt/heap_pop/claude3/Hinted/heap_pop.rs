
use std::mem;

fn pop(heap: &mut Box<Heap>) -> i64 {
    let i = heap.i[0] as i64;
    heap.n -= 1;
    swap(heap, 0, heap.n);
    down(heap, 0);
    i
}

fn swap(heap: &mut Box<Heap>, a: usize, b: usize) {
    let i = heap.i[a];
    let j = heap.i[b];
    heap.i.swap(a, b);
    let tmp = heap.a[i];
    heap.a[i] = heap.a[j];
    heap.a[j] = tmp;
}

fn down(heap: &mut Box<Heap>, mut i: usize) {
    loop {
        let left = 2 * i + 1;
        if left >= heap.n {
            break;
        }
        let mut j = left;
        if let Some(right) = left.checked_add(1) {
            if right < heap.n && !less(heap, left, right) {
                j = right;
            }
        }
        if less(heap, i, j) {
            break;
        }
        swap(heap, i, j);
        i = j;
    }
}

fn less(heap: &Box<Heap>, a: usize, b: usize) -> bool {
    heap.w[heap.i[a]] < heap.w[heap.i[b]]
}

struct Heap {
    n: usize,
    i: Vec<usize>,
    a: Vec<i64>,
    w: Vec<i64>,
}
