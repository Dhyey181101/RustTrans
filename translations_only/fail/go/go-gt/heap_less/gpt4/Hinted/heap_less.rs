
struct Heap {
    i: Vec<i64>,
    w: Vec<i64>,
}

fn less(heap: &Heap, a: i64, b: i64) -> Option<bool> {
    let i = *heap.i.get(a as usize)?;
    let j = *heap.i.get(b as usize)?;
    heap.w.get(i as usize).zip(heap.w.get(j as usize)).map(|(wi, wj)| wi < wj)
}

fn main() {
    let heap = Heap {
        i: vec![6917529238077702491, 0],
        w: vec![],
    };
    match less(&heap, 0, 1) {
        Some(result) => println!("{}", result),
        None => println!("Input is invalid, crash gracefully"),
    }

    let heap = Heap {
        i: vec![71213168960012032],
        w: vec![],
    };
    match less(&heap, 0, 1) {
        Some(result) => println!("{}", result),
        None => println!("Input is invalid, crash gracefully"),
    }
}
