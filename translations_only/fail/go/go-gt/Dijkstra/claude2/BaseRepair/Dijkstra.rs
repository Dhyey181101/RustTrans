
use std::cmp::Ordering;

struct Matrix {
    n: i64,
    a: Vec<i64>, 
}

struct Heap {
    n: i64,
    i: Vec<i64>,
    a: Vec<i64>,
    w: Vec<i64>,
}

fn dijkstra(g: &Box<Matrix>, i: i64) -> Vec<i64> {
    let mut p = vec![0; g.n as usize];
    let mut h = new_heap(g.n);
    h.w[i as usize] = 0;
    swap(&mut h, i, 0);
    
    while h.n > 0 {
        let i = pop(&mut h);
        if h.w[i as usize] == std::i64::MAX {
            return p;
        }
        update(&mut p, i, &mut h, g);
    }
    
    p
}

fn new_heap(n: i64) -> Heap {
    let mut h = Heap {
        n,
        i: vec![0; n as usize],
        a: vec![0; n as usize], 
        w: vec![std::i64::MAX; n as usize],
    };
    
    for i in 0..n {
        h.i[i as usize] = i;
        h.a[i as usize] = i;
    }
    
    h
}

fn swap(h: &mut Heap, a: i64, b: i64) {
    let i = h.i[a as usize];
    let j = h.i[b as usize];
    h.i[a as usize] = j;
    h.i[b as usize] = i;
    h.a[i as usize] = b;
    h.a[j as usize] = a;
}

fn pop(h: &mut Heap) -> i64 {
    let i = h.i[0];
    h.n -= 1; 
    swap(h, 0, h.n);
    down(h, 0);
    i  
}

fn down(h: &mut Heap, i: i64) {
    let mut i = i;
    loop {
        let left = 2 * i + 1;
        if left >= h.n {
            break;
        }
        
        let j = if left + 1 < h.n && less(h, left, left + 1) {
            left + 1
        } else {
            left
        };
        
        if less(h, i, j) {
            break;
        }
        
        swap(h, i, j);
        i = j;
    }
}

fn less(h: &Heap, a: i64, b: i64) -> bool {
    let i = h.i[a as usize];
    let j = h.i[b as usize];
    h.w[i as usize] < h.w[j as usize]
}

fn update(p: &mut Vec<i64>, i: i64, h: &mut Heap, g: &Box<Matrix>) {
    for j in 0..g.n {
        if get(g, i, j) > 0 {
            if h.w[i as usize] + get(g, i, j) < h.w[j as usize] {
                p[j as usize] = i + 1;
                h.w[j as usize] = h.w[i as usize] + get(g, i, j);
                up(h, h.a[j as usize]);
            }
        }
    }
}

fn up(h: &mut Heap, j: i64) {
    let mut j = j;
    loop {
        let i = (j - 1) / 2;
        match less(h, j, i) {
            true => break,
            false => {
                swap(h, i, j);
                j = i;
            }
        }
    }
}

fn get(g: &Box<Matrix>, i: i64, j: i64) -> i64 {
    g.a[(i * g.n + j) as usize] 
}

