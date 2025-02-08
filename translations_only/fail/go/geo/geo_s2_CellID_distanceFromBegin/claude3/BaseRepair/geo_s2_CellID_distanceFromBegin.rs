

const GEO_S2_MAX_LEVEL: u32 = 30;

fn geo_s2_cell_id_distance_from_begin(ci: Box<geo_s2_CellID>) -> i64 {
    let level = geo_s2_cell_id_level(&*ci);
    (ci.0 >> (2 * (GEO_S2_MAX_LEVEL - level) + 1)) as i64
}

fn geo_s2_cell_id_level(ci: &geo_s2_CellID) -> u32 {
    GEO_S2_MAX_LEVEL - (geo_s2_find_lsb_set_non_zero64(ci.0) >> 1)
}

fn geo_s2_find_lsb_set_non_zero64(x: u64) -> u32 {
    if x == 0 {
        0
    } else {
        x.trailing_zeros()
    }
}

struct geo_s2_CellID(u64);

