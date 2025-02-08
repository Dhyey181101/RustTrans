
const GEO_S2_MAX_LEVEL: u32 = 30;

fn geo_s2_are_siblings(a: u64, b: u64, c: u64, d: u64) -> bool {
    // A necessary (but not sufficient) condition is that the XOR of the
    // four cell IDs must be zero. This is also very fast to test.
    if (a ^ b ^ c) != d {
        return false;
    }

    // Now we do a slightly more expensive but exact test. First, compute a
    // mask that blocks out the two bits that encode the child position of
    // "id" with respect to its parent, then check that the other three
    // children all agree with "mask".
    let mask = geo_s2_lsb(d) << 1;
    let mask = !((mask) + (mask << 1));
    let id_masked = d & mask;
    (a & mask) == id_masked
        && (b & mask) == id_masked
        && (c & mask) == id_masked
        && !geo_s2_is_face(d)
}

fn geo_s2_lsb(ci: u64) -> u64 {
    ci & (!ci).wrapping_neg()
}

fn geo_s2_is_face(ci: u64) -> bool {
    (ci & (geo_s2_lsb_for_level(0) - 1)) == 0
}

fn geo_s2_lsb_for_level(level: u32) -> u64 {
    1 << (2 * (GEO_S2_MAX_LEVEL - level))
}
