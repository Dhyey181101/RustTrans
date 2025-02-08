
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn read_matrix<R: BufRead>(rd: &mut R, n: i64) -> Box<Matrix> {
    let mut m = new_matrix(n);
    for i in 0..n {
        skip(rd);
        let line = rd.lines().next().unwrap().unwrap();
        for j in 0..n {
            let line = wskip(&line);
            let x = i64::from_str(&line[..end(&line)]).unwrap();
            m.set(j, i, x);
            let line = &line[x.to_string().len()..];
        }
    }
    m
}

fn new_matrix(n: i64) -> Box<Matrix> {
    Box::new(Matrix { n, a: vec![0; (n * n) as usize] })
}

fn skip<R: BufRead>(rd: &mut R) {
    let mut b = ' ';
    while b == ' ' || b == '\t' || b == '\n' {
        b = rd.fill_buf().unwrap()[0] as char;
    }
    rd.consume(1);
}

fn wskip(s: &str) -> &str {
    for i in 0..s.len() {
        if s.chars().nth(i).unwrap() != ' ' && s.chars().nth(i).unwrap() != '\t' {
            return &s[i..];
        }
    }
    ""
}

fn read_uint(s: &str) -> (i64, usize) {
    let i = end(s);
    let x = i64::from_str(&s[..i]).unwrap();
    (x, i)
}

fn end(s: &str) -> usize {
    for i in 0..s.len() {
        if s.chars().nth(i).unwrap() == ' ' || s.chars().nth(i).unwrap() == '\t' || s.chars().nth(i).unwrap() == '\n' {
            return i;
        }
    }
    0
}

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn set(&mut self, i: i64, j: i64, v: i64) {
        self.a[(i * self.n + j) as usize] = v;
    }
}
