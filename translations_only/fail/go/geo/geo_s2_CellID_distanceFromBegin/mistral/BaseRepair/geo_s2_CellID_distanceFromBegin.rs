

use std::mem;
use std::usize;

const GEO_S2_MAX_LEVEL: usize = 30;

fn geo_s2_find_lsb_set_non_zero64(x: u64) -> usize {
if x == 0 {
return 0;
}
let mut x = x;
let mut result = 0;
while x > 0 {
x = x >> 1;
result += 1;
}
result - 1
}

