

use std::io::{BufRead, BufReader, Read};

fn read_matrix(rd: &mut BufReader<impl BufRead>, n: i64) -> Box<Matrix> {
    let mut m = Box::new(Matrix::new(n));
    let mut i = 0;
    while i < n {
        skip(rd);
        let mut line = String::new();
        rd.read_line(&mut line).unwrap();
        let mut j = 0;
        while j < n {
            line = wskip(line);
            let (x, p) = read_uint(&line);
            m.set(j, i, x);
            if p == 0 {
                panic!("bad int");
            }
            line = line[p..].to_string();
            j += 1;
        }
        i += 1;
    }
    m
}

fn skip(rd: &mut BufReader<impl BufRead>) {
    let mut b = [0u8; 1];
    loop {
        match rd.read(&mut b) {
            Ok(0) => return,
            Ok(_) => {
                if b[0] != b' ' && b[0] != b'\t' && b[0] != b'\n' {
                    rd.consume(1);
                    return;
                }
            }
            Err(_) => return,
        }
    }
}

fn wskip(s: String) -> String {
    let mut i = 0;
    while i < s.len() {
        if s.as_bytes()[i] != b' ' && s.as_bytes()[i] != b'\t' {
            return s[i..].to_string();
        }
        i += 1;
    }
    "".to_string()
}

fn read_uint(s: &str) -> (i64, usize) {
    let i = end(s);
    let x = s[..i].parse::<i64>().unwrap();
    (x, i)
}

fn end(s: &str) -> usize {
    let mut i = 0;
    while i < s.len() {
        if s.as_bytes()[i] == b' ' || s.as_bytes()[i] == b'\t' || s.as_bytes()[i] == b'\n' {
            return i;
        }
        i += 1;
    }
    0
}

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn new(n: i64) -> Matrix {
        Matrix {
            n,
            a: vec![0; (n * n) as usize],
        }
    }

    fn set(&mut self, i: i64, j: i64, v: i64) {
        self.a[(i * self.n + j) as usize] = v;
    }
}

