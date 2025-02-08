
struct Heap {
    i: Vec<i64>,
    a: Vec<i64>,
    w: Vec<i64>,
}

impl Heap {
    fn new(i: Vec<i64>, a: Vec<i64>, w: Vec<i64>) -> Self {
        Heap { i, a, w }
    }
}

fn up(heap: &mut Heap, mut j: i64) {
    while let Some(i) = ((j - 1) / 2).checked_sub(0) {
        if i == j || less(heap, i, j) {
            break;
        }
        swap(heap, i, j);
        j = i;
    }
}

fn swap(heap: &mut Heap, a: i64, b: i64) {
    let (i, j) = (heap.i[a as usize], heap.i[b as usize]);
    heap.i.swap(a as usize, b as usize);
    heap.a.swap(i as usize, j as usize);
    heap.a[i as usize] = b;
    heap.a[j as usize] = a;
}

fn less(heap: &Heap, a: i64, b: i64) -> bool {
    let (i, j) = (heap.i[a as usize], heap.i[b as usize]);
    heap.w[i as usize] < heap.w[j as usize]
}
