

use std::fmt;
use std::io::{BufReader, BufRead};
use std::str;
use std::string::String;

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

fn read_matrix(rd: &mut BufReader<std::io::StdinLock<'_>>, n: i64) -> Matrix {
    let mut m = Matrix::new(n);
    let mut i: i64 = 0;
    let mut j: i64 = 0;

    for _ in 0..n {
        let mut line = String::new();
        rd.read_line(&mut line).unwrap();

        for _ in 0..n {
            let line2 = skip_whitespace(line.clone());
            let (x, _) = read_uint(&line2);
            m.set(j, i, x);
        }
        i += 1;
    }

    m
}

fn skip_whitespace(s: String) -> String {
    s.trim_start().to_string()
}

fn read_uint(s: &str) -> (i64, i64) {
    let i = end(s);
    let x: i64 = s[..i].parse().unwrap();
    (x, i as i64)
}

fn end(s: &str) -> usize {
    s.find(|c: char| c != ' ' && c != '\t').unwrap_or(s.len())
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

fn main() {
    let stdin = std::io::stdin();
    let mut rd = BufReader::new(stdin.lock());

    let n: i64 = readline(&mut rd).trim().parse().unwrap();
    let matrix = read_matrix(&mut rd, n);

    println!("{}", matrix);
}

fn readline(rd: &mut BufReader<std::io::StdinLock<'_>>) -> String {
    let mut line = String::new();
    rd.read_line(&mut line).unwrap();
    line
}

