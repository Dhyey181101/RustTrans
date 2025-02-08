

struct Heap {
    n: i64,
    i: Vec<i64>,
    a: Vec<i64>,
    w: Vec<i64>,
}

fn down(mut h: Box<Heap>, mut i: i64) {
    loop {
        let left = 2 * i + 1;
        if left >= h.n {
            break;
        }
        let mut j = left;
        if let Some(right) = left.checked_add(1) {
            if right < h.n && less(&h, left, right) {
                j = right;
            }
        }
        if less(&h, i, j) {
            break;
        }
        swap(&mut h, i, j);
        i = j;
    }
}

fn less(h: &Heap, a: i64, b: i64) -> bool {
    let i = h.i[a as usize];
    let j = h.i[b as usize];
    h.w[i as usize] < h.w[j as usize]
}

fn swap(h: &mut Heap, a: i64, b: i64) {
    let i = h.i[a as usize];
    let j = h.i[b as usize];
    h.i.swap(a as usize, b as usize);
    h.a.swap(i as usize, j as usize);
}

