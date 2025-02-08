
use std::fmt;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str;

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn new(n: i64) -> Matrix {
        Matrix {
            n: n,
            a: vec![0; (n * n) as usize],
        }
    }

    fn set(&mut self, i: i64, j: i64, v: i64) {
        self.a[(i * self.n + j) as usize] = v;
    }
}

struct BufferReader {
    reader: BufReader<File>,
    last_byte: i8,
    last_rune_size: i64,
}

impl BufferReader {
    fn new(file: File) -> BufferReader {
        BufferReader {
            reader: BufReader::new(file),
            last_byte: -1,
            last_rune_size: -1,
        }
    }
}
