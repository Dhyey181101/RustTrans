
use std::boxed::Box;

fn down(h: &mut Box<Heap>, i: i64) {
    let mut i = i;
    loop {
        let left = 2 * i + 1;
        if left >= h.n {
            break;
        }
        let mut j = left;
        if let Some(right) = left.checked_add(1) {
            if right < h.n && !less(h, left, right) {
                j = right;
            }
        }
        if less(h, i, j) {
            break;
        }
        swap(h, i, j);
        i = j;
    }
}

fn less(h: &Box<Heap>, a: i64, b: i64) -> bool {
    let i = h.i[a as usize];
    let j = h.i[b as usize];
    h.w[i as usize] < h.w[j as usize]
}

fn swap(h: &mut Box<Heap>, a: i64, b: i64) {
    let i = h.i[a as usize];
    let j = h.i[b as usize];
    h.i.swap(a as usize, b as usize);
    h.a[i as usize] = b;
    h.a[j as usize] = a;
}

#[derive(Default)]
struct Heap {
    n: i64,
    i: Vec<i64>,
    a: Vec<i64>,
    w: Vec<i64>,
}
