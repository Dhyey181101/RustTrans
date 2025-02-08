
struct Matrix {
    n: i64,
    a: Vec<i64>,
}

struct Vector(Vec<i64>);

fn delta_part(a: &Matrix, b: &Matrix, dist: &Matrix, p: &Vector, i: i64, j: i64, r: i64, s: i64) -> i64 {
    (get(dist, i, j) + (get(a, r, i) - get(a, r, j) + get(a, s, j) - get(a, s, i))
        * (get(b, p.0[s as usize] as i64, p.0[i as usize] as i64) - get(b, p.0[s as usize] as i64, p.0[j as usize] as i64) + get(b, p.0[r as usize] as i64, p.0[j as usize] as i64) - get(b, p.0[r as usize] as i64, p.0[i as usize] as i64))
        + (get(a, i, r) - get(a, j, r) + get(a, j, s) - get(a, i, s))
            * (get(b, p.0[i as usize] as i64, p.0[s as usize] as i64) - get(b, p.0[j as usize] as i64, p.0[s as usize] as i64) + get(b, p.0[j as usize] as i64, p.0[r as usize] as i64) - get(b, p.0[i as usize] as i64, p.0[r as usize] as i64)))
}

fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m.a[(i * m.n + j) as usize]
}
