
use std::boxed::Box;

struct Vector(Box<[i64]>);

struct Matrix {
    n: i64,
    a: Box<[i64]>,
}

fn delta_part(
    a: Matrix,
    b: Matrix,
    dist: Matrix,
    p: Vector,
    i: i64,
    j: i64,
    r: i64,
    s: i64,
) -> i64 {
    (dist.a[i as usize * dist.n as usize + j as usize]
        + (a.a[r as usize * a.n as usize + i as usize]
            - a.a[r as usize * a.n as usize + j as usize]
            + a.a[s as usize * a.n as usize + j as usize]
            - a.a[s as usize * a.n as usize + i as usize])
            * (b.a[p.0[s as usize] as usize * b.n as usize + p.0[i as usize] as usize]
                - b.a[p.0[s as usize] as usize * b.n as usize + p.0[j as usize] as usize]
                + b.a[p.0[r as usize] as usize * b.n as usize + p.0[j as usize] as usize]
                - b.a[p.0[r as usize] as usize + p.0[i as usize] as usize])
        + (a.a[i as usize * a.n as usize + r as usize]
            - a.a[j as usize * a.n as usize + r as usize]
            + a.a[j as usize * a.n as usize + s as usize]
            - a.a[i as usize * a.n as usize + s as usize])
            * (b.a[p.0[i as usize] as usize * b.n as usize + p.0[s as usize] as usize]
                - b.a[p.0[j as usize] as usize * b.n as usize + p.0[s as usize] as usize]
                + b.a[p.0[j as usize] as usize * b.n as usize + p.0[r as usize] as usize]
                - b.a[p.0[i as usize] as usize * b.n as usize + p.0[r as usize] as usize]))
}
