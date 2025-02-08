
pub fn new_heap(n: i64) -> Heap {
    Heap {
        N: n,
        I: vec![0; n as usize],
        A: vec![0; n as usize],
        W: vec![i64::MAX; n as usize],
    }
}

pub struct Heap {
    pub N: i64,
    pub I: Vec<i64>,
    pub A: Vec<i64>,
    pub W: Vec<i64>,
}
