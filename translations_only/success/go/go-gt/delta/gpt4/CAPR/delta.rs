
struct Matrix {
    n: i64,
    a: Vec<i64>,
}

struct Vector(Vec<i64>);

fn delta(a: &Matrix, b: &Matrix, p: Vector, r: i64, s: i64) -> i64 {
    let mut d = (get(&a, r, r) - get(&a, s, s)) * (get(&b, p.0[s as usize], p.0[s as usize]) - get(&b, p.0[r as usize], p.0[r as usize]))
        + (get(&a, r, s) - get(&a, s, r)) * (get(&b, p.0[s as usize], p.0[r as usize]) - get(&b, p.0[r as usize], p.0[s as usize]));
    for i in 0..len(&p) {
        if i != r && i != s {
            d += (get(&a, i, r) - get(&a, i, s)) * (get(&b, p.0[i as usize], p.0[s as usize]) - get(&b, p.0[i as usize], p.0[r as usize]))
                + (get(&a, r, i) - get(&a, s, i)) * (get(&b, p.0[s as usize], p.0[i as usize]) - get(&b, p.0[r as usize], p.0[i as usize]));
        }
    }
    d
}

fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m.a[(i * m.n + j) as usize]
}

fn len(v: &Vector) -> i64 {
    v.0.len() as i64
}
