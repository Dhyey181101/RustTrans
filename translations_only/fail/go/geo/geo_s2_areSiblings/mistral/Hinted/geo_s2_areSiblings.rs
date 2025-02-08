
use std::mem;

const GEO_S2_MAX_LEVEL: u32 = 30;

type GeoS2CellID = u64;

fn geo_s2_are_siblings(a: GeoS2CellID, b: GeoS2CellID, c: GeoS2CellID, d: GeoS2CellID) -> bool {
    if (a ^ b ^ c) != d {
        return false;
    }

    let mask = (d & 3) << 1;
    let mask = !((mask as u64) + ((mask as u64) << 1));
    let id_masked = (d & mask) as u64;

    (a & mask == id_masked &&
     b & mask == id_masked &&
     c & mask == id_masked &&
     d & !(1u64 << (63 - (2 * (GEO_S2_MAX_LEVEL as u64) - 2))) != 0)
}

fn lsb(ci: GeoS2CellID) -> u64 {
    ci & ! (ci - 1)
}

fn is_face(ci: GeoS2CellID) -> bool {
    ci & ((1u64 << (64 - (2 * (GEO_S2_MAX_LEVEL as u64) - 2))) - 1) == 0
}

fn lsb_for_level(level: i32) -> u64 {
    1 << (2 * (GEO_S2_MAX_LEVEL as u64 - level as u64))
}

fn main() {
    let _ = geo_s2_are_siblings(6582955728259668827, 721240045402455899, 0, 61952);
    let _ = geo_s2_are_siblings(5822372699, 721240044061065216, 18446462598732840960, 549755813979);
}
