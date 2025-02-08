
pub fn copy_vector(v: &mut Vec<i64>, w: &Vec<i64>) {
    for i in 0..v.len() {
        v[i] = w[i];
    }
}
