

use std::convert::TryInto;

const GEO_S2_MAXLEVEL: i32 = 30;

fn distance_from_begin(ci: GeoS2CellId) -> i64 {
    ((ci.0 >> u64::try_from(2 * (GEO_S2_MAXLEVEL - level(ci)) + 1).unwrap()) as i64)  
}

fn level(ci: GeoS2CellId) -> i32 {
    GEO_S2_MAXLEVEL - (geo_s2_find_lsb_set_non_zero64(ci.0) >> 1)  
}

fn geo_s2_find_lsb_set_non_zero64(x: u64) -> i32 {
    if x == 0 {
        0
    } else {
        x.trailing_zeros() as i32
    }
}

struct GeoS2CellId(u64);

