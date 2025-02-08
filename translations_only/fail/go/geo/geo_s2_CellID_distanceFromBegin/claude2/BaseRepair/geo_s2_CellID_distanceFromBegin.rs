
use std::convert::TryInto;

struct GeoS2CellId(Box<u64>);

const GEO_S2_MAXLEVEL: i32 = 30;

fn distance_from_begin(ci: &GeoS2CellId) -> i64 {
    (*ci.0 >> (2 * (GEO_S2_MAXLEVEL - level(ci)) + 1) as u32).try_into().unwrap() 
}

fn level(ci: &GeoS2CellId) -> i32 {
    GEO_S2_MAXLEVEL - find_lsb_set_non_zero64(*ci.0) >> 1
}

fn find_lsb_set_non_zero64(x: u64) -> i32 {
    if x == 0 {
        0
    } else {
        x.trailing_zeros() as i32
    }
}

