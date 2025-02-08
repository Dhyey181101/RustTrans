

use std::boxed::Box;

const MAX_LEVEL: u32 = 30;
const POS_BITS: usize = (MAX_LEVEL * 2) as usize + 1;

type CellID = u64;

struct Cell {
id: CellID,
}

impl Cell {
fn new(face: u32) -> Self {
Cell {
id: cell_id_from_face(face),
}
}

fn children(&self, child_pos: u32) -> CellID {
let lsb = self.lsb();
let lsb_shifted = lsb >> 1;
match child_pos {
0 => self.id - lsb + (lsb >> 2),
1 => self.id - lsb + lsb_shifted,
2 => self.id - lsb_shifted + (lsb_shifted << 1),
3 => self.id - lsb_shifted + (lsb_shifted << 2),
_ => panic!(),
}
}

fn lsb(&self) -> u64 {
self.id & ! (self.id - 1)
}
}

fn cell_id_from_string(s: &str) -> CellID {
let level = s[..s.len()-2].chars().rev().take(MAX_LEVEL as usize).count() as i32 - 2;
if level < 0 || level > MAX_LEVEL as i32 {
return 0;
}
let face = (s.chars().next().unwrap() as u8 - b'0') as i32;
if face < 0 || face > 5 || s.chars().nth(1).unwrap() != '/' {
return 0;
}
let mut id = cell_id_from_face(face as u32);
for i in 2..s.len() {
let child_pos = s.chars().nth(i).unwrap() as u8 - b'0';
if child_pos > 3 {
return 0;
}
id = Box::new(Cell::new(child_pos as u32)).id;
}
id
}

fn cell_id_from_face(face: u32) -> CellID {
(1u64 << (POS_BITS as u32 * (MAX_LEVEL as u32 - face))) + lsb_for_level(0)
}

fn lsb_for_level(level: u32) -> u64 {
1 << (2 * (MAX_LEVEL - level))
}

fn main() {
println!("{}", cell_id_from_string("0/1/2/3/4/5/6/7/8/9/10/11/12/13/14/15/16/17/18/19/20/21/22/23/24/25/26/27/28/29"));
}

