

use std::io::{Read, BufRead};

struct Matrix {
    n: i64,
    a: Vec<i64>, 
}

fn matrix_new(n: i64) -> Box<Matrix> {
    Box::new(Matrix {
        n,
        a: vec![0; n as usize * n as usize],
    })
}

fn matrix_set(m: &mut Box<Matrix>, i: i64, j: i64, v: i64) {
    m.a[(i * m.n + j) as usize] = v;
}

fn read_matrix(rd: &mut impl BufRead, n: i64) -> Result<Box<Matrix>, std::io::Error> {
    let mut m = matrix_new(n);
    
    for i in 0..n {
        skip(rd)?;
        
        let mut line = String::new();
        read_line(rd, &mut line)?;
        
        for j in 0..n {
            let line_str = wskip(&line);
            
            let (x, p) = read_uint(line_str);
            matrix_set(&mut m, j, i, x);
            
            if p == 0 {
                panic!("bad int");
            }
            
            line = line[p..].to_string();
        }
    }
    
    Ok(m)
}

fn skip(rd: &mut impl BufRead) -> Result<(), std::io::Error> {
    loop {
        let mut b = [0];
        rd.read_exact(&mut b)?;
        if !(b[0] == b' ' || b[0] == b'\t' || b[0] == b'\n') {
            return Ok(());
        }
    }
}

fn read_line(rd: &mut impl BufRead, line: &mut String) -> Result<usize, std::io::Error> {
    rd.read_line(line)
}

fn wskip(s: &str) -> &str {
    s.trim_start() 
}

fn read_uint(s: &str) -> (i64, usize) {
    let i = s.find(|c: char| c == ' ' || c == '\t' || c == '\n').unwrap_or(0);
    let x = s[..i].parse().unwrap();
    (x, i)
}

