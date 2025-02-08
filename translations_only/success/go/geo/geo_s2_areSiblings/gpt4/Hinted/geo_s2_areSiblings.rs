
const GEO_S2_MAX_LEVEL: i32 = 30;

#[derive(Copy, Clone)]
struct GeoS2CellID(u64);

fn geo_s2_are_siblings(a: GeoS2CellID, b: GeoS2CellID, c: GeoS2CellID, d: GeoS2CellID) -> bool {
    if (a.0 ^ b.0 ^ c.0) != d.0 {
        return false;
    }

    let mask = lsb(d) << 1;
    let mask = !(mask + (mask << 1));
    let id_masked = d.0 & mask;
    ((a.0 & mask) == id_masked &&
     (b.0 & mask) == id_masked &&
     (c.0 & mask) == id_masked &&
     !is_face(d))
}

fn lsb(ci: GeoS2CellID) -> u64 {
    ci.0 & (!ci.0).wrapping_add(1)
}

fn is_face(ci: GeoS2CellID) -> bool {
    ci.0 & (geo_s2_lsb_for_level(0) - 1) == 0
}

fn geo_s2_lsb_for_level(level: i32) -> u64 {
    1 << (2 * (GEO_S2_MAX_LEVEL - level)) as u64
}
