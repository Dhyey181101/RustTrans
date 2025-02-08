
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn read_matrix<R: BufRead>(rd: &mut R, n: i64) -> Box<Matrix> {
    let mut m = new_matrix(n);
    for i in 0..n {
        skip(rd);
        let line = rd.lines().next().unwrap().unwrap();
        for j in 0..n {
            let line = wskip(&line);
            let (x, p) = read_uint(&line);
            m.set(j, i, x);
            if p == 0 {
                panic!("bad int");
            }
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
    s.trim_start_matches(|c| c == ' ' || c == '\t')
}

fn read_uint(s: &str) -> (i64, i64) {
    let i = end(s);
    let x = i64::from_str(&s[..i as usize]).unwrap();
    (x, i)
}

fn end(s: &str) -> i64 {
    for i in 0..s.len() as i64 {
        if s.chars().nth(i as usize).unwrap() == ' '
            || s.chars().nth(i as usize).unwrap() == '\t'
            || s.chars().nth(i as usize).unwrap() == '\n'
        {
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
