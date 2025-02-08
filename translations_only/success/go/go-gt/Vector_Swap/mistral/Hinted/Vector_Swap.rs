
use std::mem;
use std::boxed::Box;

type Vector = Box<[i64]>;

fn Swap(mut p: Vector, i: i64, j: i64) {
    if i >= p.len() as i64 || j >= p.len() as i64 {
        panic!("Input is invalid, crash gracefully");
    }
    let x = p[i as usize];
    p[i as usize] = p[j as usize];
    p[j as usize] = x;
}
