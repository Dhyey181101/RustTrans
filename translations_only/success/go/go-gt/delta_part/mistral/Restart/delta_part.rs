

use std::boxed::Box;

type Vector = Vec<i64>;
type Matrix = Box<[i64]>;

fn delta_part(a: &Matrix, b: &Matrix, dist: &Matrix, p: &Vector, i: i64, j: i64, r: i64, s: i64) -> i64 {
    return (
        dist[(i * (a.len() as i64) / 8 + j) as usize] +
            (
                a[((r * (a.len() as i64) / 8 + i as i64) as usize)] -
                    a[((r * (a.len() as i64) / 8 + j as i64) as usize)] +
                    a[((s * (a.len() as i64) / 8 + j as i64) as usize)] -
                    a[((s * (a.len() as i64) / 8 + i as i64) as usize)]
            ) *
            (
                b[p[s as usize] as usize] -
                    b[p[r as usize] as usize]
            ) +
            (
                a[((i * (a.len() as i64) / 8 + r as i64) as usize)] -
                    a[((j * (a.len() as i64) / 8 + r as i64) as usize)] +
                    a[((j * (a.len() as i64) / 8 + s as i64) as usize)] -
                    a[((i * (a.len() as i64) / 8 + s as i64) as usize)]
            )
    );
}

