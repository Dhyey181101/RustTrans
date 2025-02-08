
struct Matrix {
    n: i64,
    a: Vec<i64>,
}

struct Vector(Vec<i64>);

fn cost(a: &Matrix, b: &Matrix, p: Vector) -> i64 {
    let mut c = 0i64;
    for i in 0..p.0.len() as i64 {
        for j in 0..p.0.len() as i64 {
            c += get(&a, i, j) * get(&b, p.0[i as usize], p.0[j as usize]);
        }
    }
    c
}

fn len(v: &Vector) -> i64 {
    v.0.len() as i64
}

fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m.a[(i * m.n + j) as usize]
}
