
use std::mem;

fn pop(heap: &mut Heap) -> i64 {
    let i = heap.i[0];
    heap.n -= 1;
    swap(heap, 0, heap.n);
    down(heap, 0);
    i
}

fn swap(heap: &mut Heap, a: usize, b: usize) {
    let i = heap.i[a];
    let j = heap.i[b];
    heap.i.swap(a, b);
    let temp = heap.a[i as usize];
    heap.a[i as usize] = b as i64;
    heap.a[j as usize] = temp;
}

fn down(heap: &mut Heap, mut i: usize) {
    loop {
        let left = 2 * i + 1;
        if left >= heap.n as usize {
            break;
        }
        let mut j = left;
        if left + 1 < heap.n as usize && !less(heap, left, left + 1) {
            j = left + 1;
        }
        if less(heap, i, j) {
            break;
        }
        swap(heap, i, j);
        i = j;
    }
}

fn less(heap: &Heap, a: usize, b: usize) -> bool {
    heap.w[heap.i[a] as usize] < heap.w[heap.i[b] as usize]
}

struct Heap {
    n: usize,
    i: Vec<i64>,
    a: Vec<i64>,
    w: Vec<i64>,
}
