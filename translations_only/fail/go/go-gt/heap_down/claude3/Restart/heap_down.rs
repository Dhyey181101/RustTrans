
use std::mem;

struct Heap {
    n: i64,
    i: Box<[i64]>,
    a: Box<[i64]>,
    w: Box<[i64]>,
}

fn down(heap: &mut Heap, i: i64) {
    let mut i = i;
    loop {
        let left = 2 * i + 1;
        if left >= heap.n {
            break;
        }
        let mut j = left;
        if left + 1 < heap.n && !less(heap, left, left + 1) {
            j = left + 1;
        }
        if less(heap, i, j) {
            break;
        }
        swap(heap, i, j);
        i = j;
    }
}

fn less(heap: &Heap, a: i64, b: i64) -> bool {
    let i = heap.i[a as usize];
    let j = heap.i[b as usize];
    heap.w[i as usize] < heap.w[j as usize]
}

fn swap(heap: &mut Heap, a: i64, b: i64) {
    let i = heap.i[a as usize];
    let j = heap.i[b as usize];
    heap.i[a as usize] = heap.i[b as usize];
    heap.i[b as usize] = i;
    heap.a[i as usize] = b;
    heap.a[j as usize] = a;
}
