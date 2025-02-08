
struct Heap {
    i: Box<[i64]>,
    a: Box<[i64]>,
}

fn swap(h: &mut Heap, a: usize, b: usize) {
    let i = h.i[a];
    let j = h.i[b];
    h.i[a] = h.i[b];
    h.i[b] = i;
    h.a[i as usize] = b as i64;
    h.a[j as usize] = a as i64;
}
