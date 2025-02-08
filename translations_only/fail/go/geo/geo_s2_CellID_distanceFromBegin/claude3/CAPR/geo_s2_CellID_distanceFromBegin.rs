
const GEO_S2_MAX_LEVEL: u32 = 30;

fn geo_s2_cell_id_distance_from_begin(ci: u64) -> i64 {
    (ci >> (2 * (GEO_S2_MAX_LEVEL - geo_s2_cell_id_level(ci)) + 1)) as i64
}

fn geo_s2_cell_id_level(ci: u64) -> u32 {
    GEO_S2_MAX_LEVEL - (geo_s2_find_lsb_set_non_zero64(ci) >> 1)
}

fn geo_s2_find_lsb_set_non_zero64(x: u64) -> u32 {
    if x == 0 {
        0
    } else {
        x.trailing_zeros() as u32
    }
}

type GeoS2CellID = u64;
