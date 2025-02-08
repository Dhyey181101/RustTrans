

use std::mem;
use std::intrinsics::transmute;

const GEO_S2_MAX_LEVEL: u32 = 30;

fn geo_s2_find_lsb_set_non_zero64(x: u64) -> u32 {
if x == 0 {
0
} else {
x.leading_zeros() as u32
}
}

