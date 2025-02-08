
pub fn swap(p: &mut Vec<i64>, i: i64, j: i64) {
    let x = p[i as usize];
    p[i as usize] = p[j as usize];
    p[j as usize] = x;
}
