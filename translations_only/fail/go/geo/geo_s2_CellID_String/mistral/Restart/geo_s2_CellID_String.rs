
use std::fmt;
use std::mem;
use std::ops::Shr;
use std::str;

const NUM_FACES: u32 = 6;
const POS_BITS: u32 = 2*30 + 1;
const MAX_LEVEL: u32 = 30;

#[derive(Copy, Clone, Eq, PartialEq)]
struct CellID {
 face: u32,
 lsb: u64,
}

impl CellID {
 fn face(&self) -> u32 {
 self.face
 }

 fn lsb(&self) -> u64 {
 self.lsb
 }
}

impl fmt::Debug for CellID {
 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
 write!(f, "CellID({}, {})", self.face, self.lsb)
 }
}

fn is_valid(ci: CellID) -> bool {
 ci.face() < NUM_FACES && (ci.lsb() & 0x1555555555555555) == 0
}
