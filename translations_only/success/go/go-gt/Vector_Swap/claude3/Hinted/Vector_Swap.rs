
fn swap(p: Box<Vec<i64>>, i: i64, j: i64) {
    let mut p = p;
    let x = p[i as usize];
    p[i as usize] = p[j as usize];
    p[j as usize] = x;
}

type Vector = Vec<i64>;
