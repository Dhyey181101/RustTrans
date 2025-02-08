

use std::mem;

type CellID = u64;

const NUM_FACES: u32 = 6;
const POS_BITS: u32 = 2*30 + 1;
const MAX_LEVEL: u32 = 30;

fn is_valid(ci: CellID) -> bool {
 ci < (1 << POS_BITS) && (ci & 0x1555555555555555 != 0)
}

fn face(ci: CellID) -> u32 {
(ci >> POS_BITS) as u32
}

fn lsb(ci: CellID) -> u64 {
ci & !ci.wrapping_neg()
}

fn child_position(ci: CellID, level: u32) -> u32 {
(ci >> (2*(30-level)+1)) as u32 & 3
}

fn level(ci: CellID) -> u32 {
30 - (POS_BITS - (u64::leading_zeros(ci) as u32))/2
}

fn find_lsb_set_non_zero64(x: u64) -> u32 {
if x == 0 {
0
} else {
(64 - u64::leading_zeros(x)) as u32
}
}

