
struct Heap {
    n: i64,
    i: Vec<i64>,
    a: Vec<i64>,
    w: Vec<i64>,
}

impl Heap {
    fn new(n: i64, i: Vec<i64>, a: Vec<i64>, w: Vec<i64>) -> Self {
        Heap { n, i, a, w }
    }
}

fn down(h: &mut Heap, mut i: i64) {
    while let Some(left) = (2 * i + 1).checked_sub(1) {
        if left >= h.n {
            break;
        }
        let mut j = left;
        let right = left + 1;
        if right < h.n && !less(h, left, right) {
            j = right;
        }
        if less(h, i, j) {
            break;
        }
        swap(h, i, j);
        i = j;
    }
}

fn less(h: &Heap, a: i64, b: i64) -> bool {
    let i = h.i[a as usize] as usize;
    let j = h.i[b as usize] as usize;
    h.w[i] < h.w[j]
}

fn swap(h: &mut Heap, a: i64, b: i64) {
    let i = h.i[a as usize] as usize;
    let j = h.i[b as usize] as usize;
    h.i.swap(a as usize, b as usize);
    h.a.swap(i, j);
    let temp = h.a[i];
    h.a[i] = b;
    h.a[j] = a;
}
