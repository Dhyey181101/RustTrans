
const GEO_S2_MAX_LEVEL: i32 = 30;

pub fn geo_s2_are_siblings(a: u64, b: u64, c: u64, d: u64) -> bool {
    if (a ^ b ^ c) != d {
        return false;
    }

    let mask = (d & d.trailing_zeros() as u64) << 1;
    let mask = !(mask + (mask << 1));
    let id_masked = (d & mask);
    ((a & mask) == id_masked
        && (b & mask) == id_masked
        && (c & mask) == id_masked
        && !geo_s2_is_face(d))
}

pub fn geo_s2_lsb_for_level(level: i32) -> u64 {
    1 << (2 * (GEO_S2_MAX_LEVEL - level))
}

pub fn geo_s2_is_face(cell_id: u64) -> bool {
    (cell_id & (geo_s2_lsb_for_level(0) - 1)) == 0
}
