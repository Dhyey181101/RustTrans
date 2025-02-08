

struct Heap {
    n: i64,
    i: Vec<i64>,
    a: Vec<i64>,
    w: Vec<i64>,
}

fn down(heap: &mut Heap, mut i: i64) {
    loop {
        let left = 2 * i + 1;
        if left >= heap.n {
            break;
        }
        let mut j = left;
        if let Some(right) = left.checked_add(1) {
            if right < heap.n && less(heap, left, right) {
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

fn less(heap: &Heap, a: i64, b: i64) -> bool {
    let i = heap.i[a as usize];
    let j = heap.i[b as usize];
    heap.w[i as usize] < heap.w[j as usize]
}

fn swap(heap: &mut Heap, a: i64, b: i64) {
    let i = heap.i[a as usize];
    let j = heap.i[b as usize];
    heap.i.swap(a as usize, b as usize);
    heap.a.swap(i as usize, j as usize);
}

