
use std::num::NonZeroU64;

const GEO_S2_MAX_LEVEL: i32 = 30;

struct GeoS2CellID(u64);

fn distance_from_begin(ci: &GeoS2CellID) -> i64 {
    (ci.0 >> (2 * (GEO_S2_MAX_LEVEL - level(ci)) + 1)) as i64
}

fn level(ci: &GeoS2CellID) -> i32 {
    GEO_S2_MAX_LEVEL - (geo_s2_find_lsb_set_non_zero_64(ci.0) >> 1) as i32
}

fn geo_s2_find_lsb_set_non_zero_64(x: u64) -> u32 {
    NonZeroU64::new(x).map_or(0, |nz| nz.trailing_zeros())
}
