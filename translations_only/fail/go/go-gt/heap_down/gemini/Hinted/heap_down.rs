
fn down(h: &mut Heap, mut i: i64) {
    loop {
        let left = 2 * i + 1;
        if left >= h.N {
            break;
        }
        let mut j = left;
        if let Some(right) = left.checked_add(1) {
            if right < h.N && !less(h, left, right) {
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

fn less(h: &Heap, a: i64, b: i64) -> bool {
    let i = h.I[a as usize];
    let j = h.I[b as usize];
    h.W[i as usize] < h.W[j as usize]
}

fn swap(h: &mut Heap, a: i64, b: i64) {
    let i = h.I[a as usize];
    let j = h.I[b as usize];
    h.I[a as usize] = h.I[b as usize];
    h.I[b as usize] = i;
    h.A[i as usize] = b;
    h.A[j as usize] = a;
}

struct Heap {
    N: i64,
    I: Vec<i64>,
    A: Vec<i64>,
    W: Vec<i64>,
}
