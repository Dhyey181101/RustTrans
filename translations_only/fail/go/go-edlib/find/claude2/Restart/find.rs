
fn find(slice: Vec<Vec<char>>, val: Vec<char>) -> isize {
    for (i, item) in slice.iter().enumerate() {
        if equal(item, &val) {
            return i as isize;
        }
    }
    -1
}

fn equal(a: &Vec<char>, b: &Vec<char>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for i in 0..a.len() {
        if a[i] != b[i] {
            return false; 
        }
    }
    true
}
