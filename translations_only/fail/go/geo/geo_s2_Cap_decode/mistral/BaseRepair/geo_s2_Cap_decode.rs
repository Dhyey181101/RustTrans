
use std::io;
use std::io::Read;

struct Cap {
pub center: Box<Point>,
}

struct Decoder<R: Read> {
read_float: fn(&mut R) -> f64,
}

fn decode(c: &mut Cap, d: &mut Decoder<io::Cursor<Vec<u8>>>) {
c.center.x = (d.read_float)(&mut io::Cursor::new(vec![]));
}

struct Point {
pub x: f64,
pub y: f64,
}
