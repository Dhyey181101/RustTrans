
struct Heap {
    n: i64,
    i: Vec<i64>,
    a: Vec<i64>,
    w: Vec<i64>,
}

fn new_heap(n: i64) -> Box<Heap> {
    let mut heap = Heap {
        n,
        i: Vec::with_capacity(n as usize),
        a: Vec::with_capacity(n as usize),
        w: Vec::with_capacity(n as usize),
    };

    for i in 0..n {
        heap.i.push(i);
        heap.a.push(i);
        heap.w.push(i64::MAX);
    }

    Box::new(heap)
}
