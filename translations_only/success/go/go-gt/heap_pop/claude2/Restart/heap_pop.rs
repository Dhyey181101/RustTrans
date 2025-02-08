
use std::mem;

struct Heap {
    n: i64,
    i: Vec<i64>,
    a: Vec<i64>,
    w: Vec<i64>,
}

fn pop(h: &mut Heap) -> i64 {
    let i = h.i[0];
    h.n -= 1;
    swap(h, h.n, 0);
    down(h, 0);
    i
}

fn swap(h: &mut Heap, a: i64, b: i64) {
    let tmp = h.i[a as usize];
    h.i[a as usize] = h.i[b as usize];
    h.i[b as usize] = tmp;
    let tmp = h.a[h.i[a as usize] as usize];
    h.a[h.i[a as usize] as usize] = b;
    h.a[h.i[b as usize] as usize] = a;
}

fn down(h: &mut Heap, mut i: i64) {
    loop {
        let left = 2 * i + 1;
        if left >= h.n {
            break;
        }
        let mut j = left;
        if let Some(right) = left.checked_add(1) {
            if right < h.n && less(h, left, right) {
                j = right;
            }
        }
        if !less(h, i, j) {
            break;
        }
        swap(h, i, j);
        i = j;
    }
}

fn less(h: &Heap, a: i64, b: i64) -> bool {
    h.w[h.i[a as usize] as usize] < h.w[h.i[b as usize] as usize]
}

