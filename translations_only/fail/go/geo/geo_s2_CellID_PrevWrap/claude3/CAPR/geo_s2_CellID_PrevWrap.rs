
const GEO_S2_NUM_FACES: u64 = 6;
const GEO_S2_MAX_LEVEL: u64 = 30;
const GEO_S2_POS_BITS: u64 = 2 * GEO_S2_MAX_LEVEL + 1;
const GEO_S2_WRAP_OFFSET: u64 = GEO_S2_NUM_FACES << GEO_S2_POS_BITS;

type GeoS2CellID = u64;

fn prev_wrap(ci: GeoS2CellID) -> GeoS2CellID {
    let p = prev(ci);
    if p < GEO_S2_WRAP_OFFSET {
        return p;
    }
    return p + GEO_S2_WRAP_OFFSET;
}

fn prev(ci: GeoS2CellID) -> GeoS2CellID {
    return ci - lsb(ci) << 1;
}

fn lsb(ci: GeoS2CellID) -> GeoS2CellID {
    return ci & (!ci + 1);
}
