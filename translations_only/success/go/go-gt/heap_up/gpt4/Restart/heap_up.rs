
struct Heap {
    i: Vec<i64>,
    a: Vec<i64>,
    w: Vec<i64>,
}

impl Heap {
    fn new(i: Vec<i64>, a: Vec<i64>, w: Vec<i64>) -> Box<Self> {
        Box::new(Self { i, a, w })
    }
}

fn up(h: &mut Heap, mut j: i64) {
    loop {
        let i = (j - 1) / 2;
        if i == j || less(h, i, j) {
            break;
        }
        swap(h, i, j);
        j = i;
    }
}

fn swap(h: &mut Heap, a: i64, b: i64) {
    let (i, j) = (h.i[a as usize], h.i[b as usize]);
    h.i.swap(a as usize, b as usize);
    h.a.swap(i as usize, j as usize);
    h.a[i as usize] = b;
    h.a[j as usize] = a;
}

fn less(h: &Heap, a: i64, b: i64) -> bool {
    let (i, j) = (h.i[a as usize], h.i[b as usize]);
    h.w[i as usize] < h.w[j as usize]
}
