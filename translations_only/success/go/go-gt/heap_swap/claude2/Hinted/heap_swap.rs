
struct Heap {
    i: Vec<i64>,
    a: Vec<i64>,
}

fn swap(h: &mut Heap, a: i64, b: i64) {
    let i = h.i[a as usize];
    let j = h.i[b as usize];
    h.i[a as usize] = h.i[b as usize];
    h.i[b as usize] = i;
    h.a[i as usize] = b;
    h.a[j as usize] = a;
}
