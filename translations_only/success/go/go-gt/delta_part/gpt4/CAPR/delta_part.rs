
struct Matrix {
    n: i64,
    a: Vec<i64>,
}

struct Vector(Vec<i64>);

fn delta_part(a: &Matrix, b: &Matrix, dist: &Matrix, p: &Vector, i: i64, j: i64, r: i64, s: i64) -> i64 {
    let Vector(p) = p;
    (get(dist, i, j) + (get(a, r, i) - get(a, r, j) + get(a, s, j) - get(a, s, i))
        * (get(b, p[s as usize], p[i as usize]) - get(b, p[s as usize], p[j as usize]) + get(b, p[r as usize], p[j as usize]) - get(b, p[r as usize], p[i as usize]))
        + (get(a, i, r) - get(a, j, r) + get(a, j, s) - get(a, i, s))
        * (get(b, p[i as usize], p[s as usize]) - get(b, p[j as usize], p[s as usize]) + get(b, p[j as usize], p[r as usize]) - get(b, p[i as usize], p[r as usize])))
}

fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m.a[(i * m.n + j) as usize]
}
