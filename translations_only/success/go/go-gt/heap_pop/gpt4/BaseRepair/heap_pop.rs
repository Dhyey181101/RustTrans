
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

fn pop(h: &mut Heap) -> i64 {
    let i = h.i[0];
    h.n -= 1;
    swap(h, 0, h.n);
    down(h, 0);
    i
}

fn swap(h: &mut Heap, a: i64, b: i64) {
    let (i, j) = (h.i[a as usize], h.i[b as usize]);
    h.i.swap(a as usize, b as usize);
    h.a[i as usize] = b;
    h.a[j as usize] = a;
}

fn down(h: &mut Heap, mut i: i64) {
    loop {
        let left = 2 * i + 1;
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
    let (i, j) = (h.i[a as usize], h.i[b as usize]);
    h.w[i as usize] < h.w[j as usize]
}
