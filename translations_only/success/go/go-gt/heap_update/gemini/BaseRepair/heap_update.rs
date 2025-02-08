
use std::cmp::Ordering;

#[derive(Debug)]
struct Heap {
    n: i64,
    i: Vec<i64>,
    a: Vec<i64>,
    w: Vec<i64>,
}

#[derive(Debug)]
struct Matrix {
    n: i64,
    a: Vec<i64>,
}

fn update(heap: &mut Heap, p: &mut [i64], i: i64, g: &Matrix) {
    let mut j = 0;
    while j < g.n {
        if g.a[(i * g.n + j) as usize] > 0 {
            if heap.w[i as usize] + g.a[(i * g.n + j) as usize] < heap.w[j as usize] {
                p[j as usize] = i + 1;
                heap.w[j as usize] = heap.w[i as usize] + g.a[(i * g.n + j) as usize];
                up(heap, heap.a[j as usize]);
            }
        }
        j += 1;
    }
}

fn up(heap: &mut Heap, mut j: i64) {
    loop {
        let i = (j - 1) / 2;
        match less(heap, i, j) {
            Ordering::Equal | Ordering::Greater => break,
            Ordering::Less => {
                swap(heap, i, j);
                j = i;
            }
        }
    }
}

fn swap(heap: &mut Heap, a: i64, b: i64) {
    let i = heap.i[a as usize];
    let j = heap.i[b as usize];
    heap.i[a as usize] = heap.i[b as usize];
    heap.i[b as usize] = i;
    heap.a[i as usize] = b;
    heap.a[j as usize] = a;
}

fn less(heap: &Heap, a: i64, b: i64) -> Ordering {
    heap.w[heap.i[a as usize] as usize].cmp(&heap.w[heap.i[b as usize] as usize])
}

