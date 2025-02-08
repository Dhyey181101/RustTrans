
struct Heap {
    i: Vec<i64>,
    a: Vec<i64>,
}

fn swap(h: &mut Heap, a: i64, b: i64) {
    let (i, j) = (h.i[a as usize], h.i[b as usize]);
    h.i[a as usize] = h.i[b as usize];
    h.i[b as usize] = h.i[a as usize];
    h.a[i as usize] = b;
    h.a[j as usize] = a;
}
