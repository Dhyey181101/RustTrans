
fn swap(v: Box<Vec<i64>>, i: i64, j: i64) {
    let mut v_mut = v;
    let temp = v_mut[i as usize];
    v_mut[i as usize] = v_mut[j as usize];
    v_mut[j as usize] = temp;
}

type Vector = Vec<i64>;
