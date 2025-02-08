

use std::boxed::Box;
use std::u64;

const GEO_S2_MAX_LEVEL: u64 = 30;

type geo_s2_CellID = Box<[u64; 1]>;

fn geo_s2_are_siblings(a: geo_s2_CellID, b: geo_s2_CellID, c: geo_s2_CellID, d: geo_s2_CellID) -> bool {
    if (a[0] ^ b[0] ^ c[0]) != d[0] {
        return false;
    }
    true
}

