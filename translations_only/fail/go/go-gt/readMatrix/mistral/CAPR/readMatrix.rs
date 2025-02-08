
use std::fmt;
use std::io::{BufReader, BufRead, Read};
use std::str;
use std::vec::Vec;

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn new(n: i64) -> Matrix {
        Matrix {
            n: n,
            a: vec![0; n as usize * n as usize],
        }
    }
}

fn main() {}
