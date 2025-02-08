
const GEO_S2_MAX_LEVEL: i32 = 30;

fn geo_s2_are_siblings(a: u64, b: u64, c: u64, d: u64) -> bool {
    if (a ^ b ^ c) != d {
        return false;
    }

    let mask = (d & d.trailing_zeros() as u64) << 1;
    let mask = !(mask + (mask << 1));
    let id_masked = (d & mask);
    ((a & mask) == id_masked && (b & mask) == id_masked && (c & mask) == id_masked && !is_face(d))
}

fn lsb(ci: u64) -> u64 {
    ci & !ci + 1
}

fn is_face(ci: u64) -> bool {
    (ci & (geo_s2_lsb_for_level(0) - 1)) == 0
}

fn geo_s2_lsb_for_level(level: i32) -> u64 {
    1 << (2 * (GEO_S2_MAX_LEVEL - level)) as u64
}
