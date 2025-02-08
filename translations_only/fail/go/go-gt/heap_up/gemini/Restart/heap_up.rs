
fn up(h: &mut Heap, mut j: i64) {
    loop {
        let i = (j - 1) / 2;
        if i == j || less(h, i, j) {
            break;
        }
        swap(&mut h.I, i, j);
        swap(&mut h.A, i, j);
        swap(&mut h.W, i, j);
        j = i;
    }
}

fn swap(v: &mut Vec<i64>, a: i64, b: i64) {
    let i = v[a as usize];
    let j = v[b as usize];
    v[a as usize] = v[b as usize];
    v[b as usize] = i;
}

fn less(h: &Heap, a: i64, b: i64) -> bool {
    let i = h.I[a as usize];
    let j = h.I[b as usize];
    h.W[i as usize] < h.W[j as usize]
}

struct Heap {
    I: Vec<i64>,
    A: Vec<i64>,
    W: Vec<i64>,
}
