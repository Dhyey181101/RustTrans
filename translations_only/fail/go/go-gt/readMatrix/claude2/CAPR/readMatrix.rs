

use std::io::BufRead;
use std::io::Result;

struct Matrix {
    n: usize,
    a: Box<[i64]>,  
}

fn new_matrix(n: usize) -> Matrix {
    Matrix {
        n,
        a: Box::new([0; 0]),
    }
}

fn resize(m: &mut Matrix, n: usize) {
    let len = n * n;
    let mut a = Vec::with_capacity(len);
    for _ in 0..len {
        a.push(0);
    }
    m.a = a.into_boxed_slice();
}

fn set(m: &mut Matrix, i: usize, j: usize, v: i64) {
    m.a[i * m.n + j] = v;   
}

fn skip(rd: &mut impl BufRead) -> Result<()> {
    loop {
        let mut b = [0];
        rd.read_exact(&mut b)?;
        if !(b[0] == b' ' || b[0] == b'\t' || b[0] == b'\n') {
            return Ok(());
        }
    }
}

fn wskip(s: &str) -> &str {
    s.trim_start()   
}

fn read_uint(s: &str) -> (usize, usize) {
    let i = end(s);
    let x = s[..i].parse().unwrap();
    (x, i)  
}

fn end(s: &str) -> usize {
    s.find(|c: char| c == ' ' || c == '\t' || c == '\n').unwrap_or(0)    
}

fn read_matrix(rd: &mut impl BufRead, n: usize) -> Result<Matrix> {
    let mut m = new_matrix(n);
    resize(&mut m, n);
    let mut i = 0;
    let mut j = 0;

    while i < n {
        skip(rd)?;
        let mut line = String::new();
        rd.read_line(&mut line)?;
        j = 0;
        while j < n {
            let line_str = wskip(&line);
            let (x, p) = read_uint(line_str);
            set(&mut m, i, j, x as i64);
            if p == 0 {
                panic!("bad int");
            }
            line = line_str[p..].to_string();
            j += 1;
        }
        i += 1;
    }

    Ok(m)
}

