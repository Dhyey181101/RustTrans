

use std::mem;
use std::usize;

const GEO_S2_MAX_LEVEL: usize = 30;

fn distance_from_begin(ci: geo_s2_CellID) -> i64 {
    let shifted_ci = (ci.0 >> (2 * (GEO_S2_MAX_LEVEL - level(ci)) + 1)) as i64;
    shifted_ci
}

fn level(ci: geo_s2_CellID) -> usize {
    let lsb_set_non_zero64 = find_lsb_set_non_zero64(ci.0);
    GEO_S2_MAX_LEVEL - (lsb_set_non_zero64 >> 1)
}

fn find_lsb_set_non_zero64(x: u64) -> usize {
    if x == 0 {
        0
    } else {
        mem::size_of::<u64>() * 8 - x.leading_zeros() as usize
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct geo_s2_CellID(pub u64);

