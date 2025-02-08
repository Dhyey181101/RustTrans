
use std::io::BufRead;
use std::io::SeekFrom;
use std::io::Seek;

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

fn new_matrix(n: i64) -> Matrix {
    Matrix {
        n,
        a: vec![0; n as usize * n as usize],
    }
}

fn set(m: &mut Matrix, i: i64, j: i64, v: i64) {
    m.a[(i * m.n + j) as usize] = v; 
}

fn read_matrix(rd: &mut (impl BufRead + Seek), n: i64) -> Result<Matrix, std::io::Error> {
    let mut m = new_matrix(n);
    let mut i = 0;
    let mut j = 0;

    while i < n {
        skip(rd)?;
        let mut line = String::new();
        rd.read_line(&mut line)?;
        j = 0;
        while j < n {
            let line_str = line.as_str();
            let line_ref = line_str;
            let line_ref = wskip(line_ref);
            let (x, p) = read_uint(line_ref);
            set(&mut m, j, i, x);
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

fn skip(rd: &mut (impl BufRead + Seek)) -> Result<(), std::io::Error> {
    loop {
        let mut b = [0];
        rd.read_exact(&mut b)?;
        if !(b[0] == b' ' || b[0] == b'\t' || b[0] == b'\n') {
            rd.seek(SeekFrom::Current(-1))?;
            return Ok(());
        }
    }
}

fn wskip(s: &str) -> &str {
    s.trim_start()
}

fn read_uint(s: &str) -> (i64, usize) {
    let i = end(s);
    let x = s[..i].parse().unwrap();
    (x, i)  
}

fn end(s: &str) -> usize {
    s.find(|c: char| c == ' ' || c == '\t' || c == '\n').unwrap_or(0)
}

