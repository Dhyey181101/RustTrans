
fn find(slice: &Vec<Vec<char>>, val: &Vec<char>) -> isize {
    for (i, item) in slice.iter().enumerate() {
        if item == val {
            return i as isize;
        }
    }
    return -1;
}

fn equal(a: &Vec<char>, b: &Vec<char>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for (i, v) in a.iter().enumerate() {
        if *v != b[i] {
            return false;
        }
    }
    return true;
}
