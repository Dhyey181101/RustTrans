
pub fn swap(h: &mut Heap, a: i64, b: i64) {
    let i = h.I[a as usize];
    let j = h.I[b as usize];
    h.I[a as usize] = h.I[b as usize];
    h.I[b as usize] = i;
    h.A[i as usize] = b;
    h.A[j as usize] = a;
}

pub struct Heap {
    pub I: Vec<i64>,
    pub A: Vec<i64>,
}
