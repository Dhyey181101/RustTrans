

use std::fmt;
use std::io::{BufReader, BufRead};
use std::str;
use std::usize;
use std::fs::File;

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
}

impl BufferReader {
    fn new(filename: &str) -> BufferReader {
        BufferReader {
            reader: BufReader::new(File::open(filename).unwrap()),
        }
    }

    fn skip(&mut self) {
        let mut b: Option<u8> = None;
        loop {
            match b {
                None => b = Some(self.reader.fill_buf().unwrap()[0]),
                Some(b) => {
                    if b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' {
                        self.reader.consume(1);
                        return;
                    }
                }
            }
        }
    }

    fn read_uint(&mut self, s: String) -> (i64, usize) {
        let i: usize = s.find(' ').unwrap_or(s.len());
        let x: i64 = s[..i].parse::<i64>().unwrap();
        (x, i)
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.n {
            for j in 0..self.n {
                write!(f, "{} ", self.a[i as usize * self.n as usize + j as usize])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn read_matrix(rd: &mut BufferReader, n: i64) -> Matrix {
    let mut m = Matrix::new(n);
    let mut i: i64 = 0;
    let mut j: i64 = 0;
    for _ in 0..n {
        rd.skip();
        let mut line = String::new();
        rd.reader.read_line(&mut line).unwrap();
        for _ in 0..n {
            let (x, p) = rd.read_uint(line.clone());
            m.set(j, i, x);
            line = line[p..].to_string();
            j += 1;
        }
        i += 1;
        j = 0;
    }
    m
}

fn main() {
    let mut rd = BufferReader::new("input.txt");
    let m = read_matrix(&mut rd, 3);
    println!("{}", m);
}

