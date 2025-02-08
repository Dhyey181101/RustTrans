
use std::io::{self, BufRead};

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn new(n: i64) -> Box<Matrix> {
        Box::new(Matrix {
            n,
            a: vec![0; (n * n) as usize],
        })
    }

    fn set(&mut self, i: i64, j: i64, v: i64) {
        let index = (i * self.n + j) as usize;
        self.a[index] = v;
    }
}

fn read_matrix(rd: &mut impl BufRead, n: i64) -> Box<Matrix> {
    let mut m = Matrix::new(n);
    for i in 0..n {
        skip(rd);
        let mut line = String::new();
        rd.read_line(&mut line).unwrap();
        for j in 0..n {
            line = wskip(&line);
            let (x, p) = read_uint(&line);
            m.set(j, i, x);
            if p == 0 {
                panic!("bad int");
            }
            line = line[p as usize..].to_string();
        }
    }
    m
}

fn skip(rd: &mut impl BufRead) {
    let mut buffer = [0; 1];
    loop {
        rd.read_exact(&mut buffer).unwrap();
        match buffer[0] {
            b' ' | b'\t' | b'\n' => continue,
            _ => {
                rd.consume(0); // Simulate UnreadByte by not consuming the byte
                break;
            }
        }
    }
}

fn wskip(s: &str) -> String {
    s.trim_start_matches(|c: char| c == ' ' || c == '\t').to_string()
}

fn read_uint(s: &str) -> (i64, i64) {
    let i = end(s);
    let x = s[..i as usize].parse::<i64>().unwrap();
    (x, i)
}

fn end(s: &str) -> i64 {
    for (i, c) in s.char_indices() {
        if c == ' ' || c == '\t' || c == '\n' {
            return i as i64;
        }
    }
    0
}
