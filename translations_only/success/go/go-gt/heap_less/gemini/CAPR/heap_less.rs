
pub fn less(h: &Heap, a: i64, b: i64) -> bool {
    let i = h.I[a as usize];
    let j = h.I[b as usize];
    h.W[i as usize] < h.W[j as usize]
}

pub struct Heap {
    pub I: Vec<i64>,
    pub W: Vec<i64>,
}
