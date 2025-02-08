
struct Heap {
    i: Vec<i64>,
    w: Vec<i64>,
}

impl Heap {
    fn new(i: Vec<i64>, w: Vec<i64>) -> Box<Self> {
        Box::new(Self { i, w })
    }
}

fn less(heap: &Heap, a: i64, b: i64) -> bool {
    let i = heap.i[a as usize];
    let j = heap.i[b as usize];
    heap.w[i as usize] < heap.w[j as usize]
}

fn main() {}
