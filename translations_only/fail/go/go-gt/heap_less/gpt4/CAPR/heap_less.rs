
struct Heap {
    i: Vec<i64>,
    w: Vec<i64>,
}

impl Heap {
    fn new(i: Vec<i64>, w: Vec<i64>) -> Box<Self> {
        Box::new(Self { i, w })
    }
}

fn less(h: &Heap, a: i64, b: i64) -> bool {
    let i = h.i[a as usize];
    let j = h.i[b as usize];
    h.w[i as usize] < h.w[j as usize]
}

fn main() {}
