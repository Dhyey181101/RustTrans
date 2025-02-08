

use std::io::Read;
use std::io::Seek;
use std::io::BufRead;

struct Matrix {
    n: usize,
    a: Box<[i64]>,
}

const N: usize = 10;

fn new_matrix() -> Matrix {
    Matrix {
        n: N,
        a: Box::new([0; N * N]),
    }
}

fn set(m: &mut Matrix, i: usize, j: usize, v: i64) {
    m.a[i * m.n + j] = v;
}

fn read_matrix(rd: &mut (impl Read + Seek + BufRead)) -> Result<Matrix, std::io::Error> {
    let mut m = new_matrix();
    let mut i = 0;
    let mut j = 0;

    while i < N {
        skip(rd)?;
        let mut line = String::new();
        read_line(rd, &mut line)?;
        j = 0;
        while j < N {
            line = wskip(&line).to_string();
            let (x, p) = read_uint(&line);
            set(&mut m, j, i, x);
            if p == 0 {
                panic!("bad int");
            }
            line = line[p..].to_string();
            j += 1;
        }
        i += 1;
    }

    Ok(m)
}

fn skip(rd: &mut (impl Read + Seek)) -> Result<(), std::io::Error> {
    loop {
        let mut b = [0];
        read_exact(rd, &mut b)?;
        if !(b[0] == b' ' || b[0] == b'\t' || b[0] == b'\n') {
            seek(rd, std::io::SeekFrom::Current(-1))?;
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

fn read_line(rd: &mut (impl Read + BufRead), line: &mut String) -> Result<usize, std::io::Error> {
    rd.read_line(line)
}

fn read_exact(rd: &mut impl Read, buf: &mut [u8]) -> Result<(), std::io::Error> {
    rd.read_exact(buf)
}

fn seek(rd: &mut impl Seek, pos: std::io::SeekFrom) -> Result<u64, std::io::Error> {
    rd.seek(pos)
}

