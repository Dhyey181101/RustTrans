
const GEO_S2_MAX_LEVEL: i32 = 30;

pub fn geo_s2_are_siblings(a: u64, b: u64, c: u64, d: u64) -> bool {
    if (a ^ b ^ c) != d {
        return false;
    }

    let mask = lsb(d) << 1;
    let mask = !(mask + (mask << 1));
    let id_masked = (d & mask);
    ((a & mask) == id_masked && (b & mask) == id_masked && (c & mask) == id_masked && !is_face(d))
}

pub fn lsb(ci: u64) -> u64 {
    ci & ci.wrapping_neg()
}

pub fn is_face(ci: u64) -> bool {
    (ci & (lsb_for_level(0) - 1)) == 0
}

pub fn lsb_for_level(level: i32) -> u64 {
    1 << (2 * (GEO_S2_MAX_LEVEL - level))
}
